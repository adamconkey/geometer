use clap::{Parser, ValueEnum};
use itertools::Itertools;
use random_color::RandomColor;

use geometer::{
    convex_hull::{
        ConvexHullComputer, ConvexHullTracer, ConvexHullTracerStep, GrahamScan, Incremental,
        QuickHull,
    },
    error::FileError,
    geometry::Geometry,
    polygon::Polygon,
    triangulation::{EarClipping, Triangulation, TriangulationComputer},
    util::load_polygon,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Visualization {
    ConvexHull,
    ConvexHullGrahamScan,
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
        let rec = rerun::RecordingStreamBuilder::new(name).connect_grpc()?;
        Ok(RerunVisualizer { rec })
    }

    pub fn visualize_polygon(
        &self,
        polygon: &Polygon,
        name: &String,
        vertex_radius: Option<f32>,
        vertex_color: Option<[u8; 4]>,
        edge_radius: Option<f32>,
        edge_color: Option<[u8; 4]>,
        frame: Option<i64>,
        draw_order: Option<f32>,
    ) -> Result<(), VisualizationError> {
        if let Some(value) = frame {
            self.rec.set_time_sequence("frame", value);
        }

        let vertex_radius = vertex_radius.unwrap_or(1.0);
        let vertex_color = vertex_color.unwrap_or(RandomColor::new().to_rgba_array());
        let draw_order = draw_order.unwrap_or(30.0);

        self.rec.log(
            format!("{}/vertices", name),
            &self
                .polygon_to_rerun_points(polygon)
                .with_radii([vertex_radius])
                .with_colors([vertex_color])
                .with_draw_order(draw_order),
        )?;

        let edge_radius = edge_radius.unwrap_or(0.1);
        let edge_color = edge_color.unwrap_or(RandomColor::new().to_rgba_array());
        self.rec.log(
            format!("{}/edges", name),
            &self
                .polygon_to_rerun_edges(polygon)
                .with_radii([edge_radius])
                .with_colors([edge_color])
                // Want edges always below vertices
                .with_draw_order(draw_order - 1.0),
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

        let _ = self.visualize_polygon(polygon, &name, None, None, None, None, None, None);

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
            None,
        );

        Ok(())
    }

    pub fn visualize_convex_hull_graham_scan(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        let tracer = &mut Some(ConvexHullTracer::default());
        let _final_hull = GrahamScan.convex_hull(polygon, tracer);
        println!("{:?}", tracer);

        // Show nominal polygon for hull to be overlayed on
        let polygon_color = [132, 90, 109, 255];
        self.visualize_polygon(
            polygon,
            &format!("{name}/polygon"),
            Some(0.5),
            Some(polygon_color),
            None,
            Some(polygon_color),
            Some(0),
            Some(10.0),
        )?;

        let mut frame: i64 = 1;
        let hull_color = [25, 100, 126, 255];

        // Show initial vertex establishing min angle order
        let id_0 = polygon.vertex_ids()[0];
        let v_0 = polygon.get_vertex(&id_0).unwrap();
        self.rec.log(
            format!("{name}/alg_init/init_vertex"),
            &rerun::Points2D::new([(v_0.x as f32, v_0.y as f32)])
                .with_radii([1.0])
                .with_colors([[255, 255, 255]])
                .with_draw_order(100.0),
        )?;

        let mut prev_step: Option<&ConvexHullTracerStep> = None;
        for (i, step) in tracer.as_ref().unwrap().steps.iter().enumerate() {
            if i == 0 {
                // Show initial edge of hull
                let id_1 = step.hull[step.hull.len() - 1];
                let id_2 = step.hull[step.hull.len() - 2];
                let v_1 = polygon.get_vertex(&id_1).unwrap();
                let v_2 = polygon.get_vertex(&id_2).unwrap();
                self.rec.log(
                    format!("{name}/hull_{i}/edges"),
                    &rerun::LineStrips2D::new([[
                        (v_1.x as f32, v_1.y as f32),
                        (v_2.x as f32, v_2.y as f32),
                    ]])
                    .with_radii([0.2])
                    .with_colors([hull_color]),
                )?;
                self.rec.log(
                    format!("{name}/hull_{i}/vertices"),
                    &rerun::Points2D::new([
                        (v_1.x as f32, v_1.y as f32),
                        (v_2.x as f32, v_2.y as f32),
                    ])
                    .with_radii([0.8])
                    .with_colors([hull_color]),
                )?;
            } else {
                frame += 1;
                self.rec.set_time_sequence("frame", frame);

                // Show highlighted edge used for angle test
                let prev_hull = prev_step
                    .expect("Prev step should exist i > 0")
                    .hull
                    .clone();
                let id_head = prev_hull[prev_hull.len() - 1];
                let id_origin = prev_hull[prev_hull.len() - 2];
                let v_head = polygon.get_vertex(&id_head).unwrap();
                let v_origin = polygon.get_vertex(&id_origin).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/check_edge"),
                    &rerun::Arrows2D::from_vectors([(
                        (v_head.x - v_origin.x) as f32,
                        (v_head.y - v_origin.y) as f32,
                    )])
                    .with_origins([(v_origin.x as f32, v_origin.y as f32)])
                    .with_radii([0.3])
                    .with_colors([[242, 192, 53]])
                    .with_draw_order(100.0),
                )?;

                // Show next vertex used for angle test
                let n_id = step.next_vertex.expect("Next vertex should exist i > 0");
                let n_v = polygon.get_vertex(&n_id).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/next_vertex"),
                    &rerun::Points2D::new([(n_v.x as f32, n_v.y as f32)])
                        .with_radii([1.0])
                        .with_colors([[242, 192, 53]])
                        .with_draw_order(100.0),
                )?;

                frame += 1;
                self.rec.set_time_sequence("frame", frame);

                self.rec
                    .log(format!("{name}/alg_{i}/check_edge"), &rerun::Clear::flat())?;
                self.rec
                    .log(format!("{name}/alg_{i}/next_vertex"), &rerun::Clear::flat())?;

                let top_id = step.hull[step.hull.len() - 1];
                if n_id == top_id {
                    // Hull is fully repaired at this point, show final edge
                    // on stack connected to next vertex is a left turn
                    let id_1 = top_id;
                    let id_2 = step.hull[step.hull.len() - 2];
                    let id_3 = step.hull[step.hull.len() - 3];
                    let v_1 = polygon.get_vertex(&id_1).unwrap();
                    let v_2 = polygon.get_vertex(&id_2).unwrap();
                    let v_3 = polygon.get_vertex(&id_3).unwrap();
                    self.rec.log(
                        format!("{name}/alg_{i}/valid/edges"),
                        &rerun::LineStrips2D::new([[
                            (v_1.x as f32, v_1.y as f32),
                            (v_2.x as f32, v_2.y as f32),
                            (v_3.x as f32, v_3.y as f32),
                        ]])
                        .with_radii([0.3])
                        .with_colors([[52, 163, 82]])
                        .with_draw_order(99.0),
                    )?;
                    self.rec.log(
                        format!("{name}/alg_{i}/valid/vertices"),
                        &rerun::Points2D::new([
                            (v_1.x as f32, v_1.y as f32),
                            (v_2.x as f32, v_2.y as f32),
                            (v_3.x as f32, v_3.y as f32),
                        ])
                        .with_radii([1.0])
                        .with_colors([[52, 163, 82]])
                        .with_draw_order(100.0),
                    )?;
                } else {
                    // Render final edge on stack to next vertex as right turn
                    let prev_hull = prev_step.expect("Prev step exists for i > 0").hull.clone();
                    let id_1 = n_id;
                    let id_2 = prev_hull[prev_hull.len() - 1];
                    let id_3 = prev_hull[prev_hull.len() - 2];
                    let v_1 = polygon.get_vertex(&id_1).unwrap();
                    let v_2 = polygon.get_vertex(&id_2).unwrap();
                    let v_3 = polygon.get_vertex(&id_3).unwrap();
                    self.rec.log(
                        format!("{name}/alg_{i}/invalid/edges"),
                        &rerun::LineStrips2D::new([[
                            (v_1.x as f32, v_1.y as f32),
                            (v_2.x as f32, v_2.y as f32),
                            (v_3.x as f32, v_3.y as f32),
                        ]])
                        .with_radii([0.3])
                        .with_colors([[163, 0, 0]])
                        .with_draw_order(99.0),
                    )?;
                    self.rec.log(
                        format!("{name}/alg_{i}/invalid/vertices"),
                        &rerun::Points2D::new([
                            (v_1.x as f32, v_1.y as f32),
                            (v_2.x as f32, v_2.y as f32),
                            (v_3.x as f32, v_3.y as f32),
                        ])
                        .with_radii([1.0])
                        .with_colors([[163, 0, 0]])
                        .with_draw_order(100.0),
                    )?;
                }

                // Show computed hull for this step
                frame += 1;
                self.visualize_polygon(
                    &polygon.get_polygon(step.hull.clone(), false, false),
                    &format!("{name}/hull_{i}"),
                    Some(0.8),
                    Some(hull_color),
                    Some(0.2),
                    Some(hull_color),
                    Some(frame),
                    None,
                )?;
            }
            prev_step = Some(step);

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
            None,
            Some(polygon_color),
            Some(0),
            Some(10.0),
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
                &polygon.get_polygon(step.hull.clone(), false, false),
                &format!("{name}/hull_{i}"),
                Some(0.8),
                Some(hull_color),
                None,
                Some(hull_color),
                Some(frame),
                Some(50.0),
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

    fn polygon_to_rerun_points(&self, polygon: &Polygon) -> rerun::Points2D {
        rerun::Points2D::new(
            polygon
                .vertices()
                .into_iter()
                .map(|v| (v.x as f32, v.y as f32)),
        )
    }

    fn polygon_to_rerun_edges(&self, polygon: &Polygon) -> rerun::LineStrips2D {
        let mut edge_points = polygon
            .vertices()
            .into_iter()
            .map(|v| (v.x as f32, v.y as f32))
            .collect_vec();
        edge_points.push(edge_points[0]);
        rerun::LineStrips2D::new([edge_points])
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
        Visualization::ConvexHullGrahamScan => {
            visualizer?.visualize_convex_hull_graham_scan(&polygon, &name)?
        }
        Visualization::ConvexHullIncremental => {
            visualizer?.visualize_convex_hull_incremental(&polygon, &name)?
        }
        Visualization::Triangulation => visualizer?.visualize_triangulation(&polygon, &name)?,
    };

    Ok(())
}
