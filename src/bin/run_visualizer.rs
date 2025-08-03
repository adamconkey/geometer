use clap::{Parser, ValueEnum};
use itertools::Itertools;
use random_color::RandomColor;

use geometer::{
    convex_hull::{ConvexHullComputer, ConvexHullTracer, Incremental, QuickHull},
    error::FileError,
    geometry::Geometry,
    polygon::Polygon,
    triangulation::{EarClipping, Triangulation, TriangulationComputer},
    util::load_polygon,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Visualization {
    ConvexHull,
    ConvexHullIncremental,
    Triangulation,
}

/// Visualize polygons and algorithms using Rerun.io``
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder containing polygon file
    #[arg(short, long, default_value = "interesting_polygon_archive")]
    folder: String,

    /// Polygon name to visualize (without the .json extension)
    #[arg(short, long, default_value = "skimage_horse")]
    polygon: String,

    /// Type of visualization to generate
    #[arg(short, long, value_enum)]
    visualization: Visualization,
}

#[derive(Debug)]
pub enum VisualizationError {
    File(FileError),
    Rerun(rerun::RecordingStreamError),
}

impl From<FileError> for VisualizationError {
    fn from(value: FileError) -> Self {
        VisualizationError::File(value)
    }
}

impl From<rerun::RecordingStreamError> for VisualizationError {
    fn from(value: rerun::RecordingStreamError) -> Self {
        VisualizationError::Rerun(value)
    }
}

pub struct RerunVisualizer {
    rec: rerun::RecordingStream,
}

impl RerunVisualizer {
    pub fn new(name: String) -> Result<Self, VisualizationError> {
        let rec = rerun::RecordingStreamBuilder::new(name).connect_tcp()?;
        Ok(RerunVisualizer { rec })
    }

    pub fn visualize_polygon(
        &self,
        polygon: &Polygon,
        name: &String,
        vertex_radius: f32,
        vertex_color: Option<[u8; 3]>,
        edge_color: Option<[u8; 3]>,
        frame: Option<i64>,
    ) -> Result<(), VisualizationError> {
        if let Some(value) = frame {
            self.rec.set_time_sequence("frame", value);
        }

        let vertex_color = vertex_color.unwrap_or(RandomColor::new().to_rgb_array());
        self.rec.log(
            format!("{}/vertices", name),
            &self
                .polygon_to_rerun_points(polygon)
                .with_radii([vertex_radius])
                .with_colors([vertex_color]),
        )?;

        let edge_color = edge_color.unwrap_or(RandomColor::new().to_rgb_array());
        self.rec.log(
            format!("{}/edges", name),
            &self
                .polygon_to_rerun_edges(polygon)
                .with_colors([edge_color]),
        )?;

        Ok(())
    }

    pub fn visualize_triangulation(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        let name = format!("{name}/triangulation");
        let triangulation = EarClipping.triangulation(polygon);
        let rerun_meshes = self.triangulation_to_rerun_meshes(&triangulation, polygon);

        let _ = self.visualize_polygon(polygon, &name, 5.0, None, None, None);

        for (i, mesh) in rerun_meshes.iter().enumerate() {
            self.rec.log(format!("{}/triangle_{}", &name, i), mesh)?;
        }

        Ok(())
    }

    pub fn visualize_convex_hull(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        let _ = self.visualize_polygon(polygon, &format!("{name}/polygon"), 5.0, None, None, None);

        let hull = QuickHull.convex_hull(polygon, &mut None);
        let _ = self.visualize_polygon(
            &hull,
            &format!("{name}/convex_hull"),
            10.0,
            None,
            None,
            None,
        );

        Ok(())
    }

    pub fn visualize_convex_hull_incremental(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        let tracer = &mut Some(ConvexHullTracer::default());
        let _final_hull = Incremental.convex_hull(polygon, tracer);
        println!("{:?}", tracer);

        let color = [242, 192, 53];
        for (i, step) in tracer.as_ref().unwrap().steps.iter().enumerate() {
            let _ = self.visualize_polygon(
                &step.hull,
                &format!("{name}/hull_{i}"),
                1.0,
                Some(color),
                Some(color),
                Some(i.try_into().unwrap()),
            );
        }

        Ok(())
    }

    fn polygon_to_rerun_points(&self, polygon: &Polygon) -> rerun::Points3D {
        rerun::Points3D::new(
            polygon
                .vertices()
                .into_iter()
                // .sorted_by_key(|v| v.id)
                .map(|v| (v.x as f32, v.y as f32, 0.0)),
        )
    }

    fn polygon_to_rerun_edges(&self, polygon: &Polygon) -> rerun::LineStrips3D {
        let mut edge_points = polygon
            .vertices()
            .into_iter()
            // .sorted_by_key(|v| v.id)
            .map(|v| (v.x as f32, v.y as f32, 0.0))
            .collect_vec();
        edge_points.push(edge_points[0]);
        rerun::LineStrips3D::new([edge_points])
    }

    fn triangulation_to_rerun_meshes(
        &self,
        triangulation: &Triangulation,
        polygon: &Polygon,
    ) -> Vec<rerun::Mesh3D> {
        let mut meshes = Vec::new();
        for ids in triangulation.iter() {
            let color = RandomColor::new().to_rgb_array();
            let t = polygon.get_triangle(&ids.0, &ids.1, &ids.2).unwrap();
            let points = [
                [t.v1.x as f32, t.v1.y as f32, 0.0],
                [t.v2.x as f32, t.v2.y as f32, 0.0],
                [t.v3.x as f32, t.v3.y as f32, 0.0],
            ];
            let mesh = rerun::Mesh3D::new(points).with_vertex_colors([color, color, color]);
            meshes.push(mesh);
        }
        meshes
    }
}

fn main() -> Result<(), VisualizationError> {
    let args = Args::parse();
    let visualizer = RerunVisualizer::new("Geometer".to_string());

    let polygon = load_polygon(&args.polygon, &args.folder)?;
    let name = format!("{}/{}", args.polygon, args.folder);

    match args.visualization {
        Visualization::ConvexHull => visualizer?.visualize_convex_hull(&polygon, &name)?,
        Visualization::ConvexHullIncremental => {
            visualizer?.visualize_convex_hull_incremental(&polygon, &name)?
        }
        Visualization::Triangulation => visualizer?.visualize_triangulation(&polygon, &name)?,
    };

    Ok(())
}
