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
        vertex_radius: Option<f32>,
        vertex_color: Option<[u8; 4]>,
        edge_color: Option<[u8; 4]>,
        frame: Option<i64>,
        z: Option<f32>,
    ) -> Result<(), VisualizationError> {
        if let Some(value) = frame {
            self.rec.set_time_sequence("frame", value);
        }

        let vertex_radius = vertex_radius.unwrap_or(1.0);
        let vertex_color = vertex_color.unwrap_or(RandomColor::new().to_rgba_array());

        self.rec.log(
            format!("{}/vertices", name),
            &self
                .polygon_to_rerun_points(polygon, z)
                .with_radii([vertex_radius])
                .with_colors([vertex_color]),
        )?;

        let edge_color = edge_color.unwrap_or(RandomColor::new().to_rgba_array());
        self.rec.log(
            format!("{}/edges", name),
            &self
                .polygon_to_rerun_edges(polygon, z)
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

        let _ = self.visualize_polygon(polygon, &name, None, None, None, None, None);

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
        let _ = self.visualize_polygon(
            polygon,
            &format!("{name}/polygon"),
            None,
            None,
            None,
            None,
            None,
        );

        let hull = QuickHull.convex_hull(polygon, &mut None);
        let _ = self.visualize_polygon(
            &hull,
            &format!("{name}/convex_hull"),
            None,
            None,
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

        // Show nominal polygon for hull to be overlayed on
        let polygon_color = [71, 121, 230, 100];
        self.visualize_polygon(
            polygon,
            &format!("{name}/polygon"),
            Some(0.5),
            Some(polygon_color),
            Some(polygon_color),
            Some(0),
            None,
        )?;

        let mut frame: i64 = 1;
        let hull_color = [242, 192, 53, 255];

        // For each step will show upper/lower tangent vertex selection and
        // how they connect to the current hull, followed by the resulting
        // hull computed at that step
        for (i, step) in tracer.as_ref().unwrap().steps.iter().enumerate() {
            if i > 0 {
                frame += 1;
                self.rec.set_time_sequence("frame", frame);

                // Show upper/lower tangent vertices and their connection
                // to the current hull
                let ut_id = step
                    .upper_tangent_vertex
                    .expect("Upper tangent vertex should exist i > 0");
                let ut_v = polygon.get_vertex(&ut_id).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/upper_tangent_vertex"),
                    &rerun::Points3D::new([(ut_v.x as f32, ut_v.y as f32, 0.1)])
                        .with_radii([1.0])
                        .with_colors([[255, 0, 0]]),
                )?;

                let lt_id = step
                    .lower_tangent_vertex
                    .expect("Lower tangent vertex should exist i > 0");
                let lt_v = polygon.get_vertex(&lt_id).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/lower_tangent_vertex"),
                    &rerun::Points3D::new([(lt_v.x as f32, lt_v.y as f32, 0.1)])
                        .with_radii([1.0])
                        .with_colors([[0, 255, 0]]),
                )?;

                let n_id = step.next_vertex.expect("Next vertex should exist i > 0");
                let n_v = polygon.get_vertex(&n_id).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/next_vertex"),
                    &rerun::Points3D::new([(n_v.x as f32, n_v.y as f32, 0.1)])
                        .with_radii([1.0])
                        .with_colors([[0, 0, 255]]),
                )?;

                self.rec.log(
                    format!("{name}/alg_{i}/upper_tangent"),
                    &rerun::LineStrips3D::new([[
                        (ut_v.x as f32, ut_v.y as f32, 0.1),
                        (n_v.x as f32, n_v.y as f32, 0.1),
                    ]])
                    .with_radii([0.2])
                    .with_colors([[255, 0, 0]]),
                )?;
                self.rec.log(
                    format!("{name}/alg_{i}/lower_tangent"),
                    &rerun::LineStrips3D::new([[
                        (lt_v.x as f32, lt_v.y as f32, 0.1),
                        (n_v.x as f32, n_v.y as f32, 0.1),
                    ]])
                    .with_radii([0.2])
                    .with_colors([[0, 0, 255]]),
                )?;
            }

            // Show computed hull for this step
            frame += 1;
            self.visualize_polygon(
                &step.hull,
                &format!("{name}/hull_{i}"),
                Some(0.8),
                Some(hull_color),
                Some(hull_color),
                Some(frame),
                Some(0.1),
            )?;
            self.rec
                .log(format!("{name}/alg_{i}"), &rerun::Clear::recursive())?;

            // Clear out old hull visualizations
            if i > 0 {
                self.rec.log(
                    format!("{}/hull_{}", name, i - 1),
                    &rerun::Clear::recursive(),
                )?;
            }
        }

        Ok(())
    }

    fn polygon_to_rerun_points(&self, polygon: &Polygon, z: Option<f32>) -> rerun::Points3D {
        rerun::Points3D::new(
            polygon
                .vertices()
                .into_iter()
                // .sorted_by_key(|v| v.id)
                .map(|v| (v.x as f32, v.y as f32, z.unwrap_or(0.0))),
        )
    }

    fn polygon_to_rerun_edges(&self, polygon: &Polygon, z: Option<f32>) -> rerun::LineStrips3D {
        let mut edge_points = polygon
            .vertices()
            .into_iter()
            // .sorted_by_key(|v| v.id)
            .map(|v| (v.x as f32, v.y as f32, z.unwrap_or(0.0)))
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
