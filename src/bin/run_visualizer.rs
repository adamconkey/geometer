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
    vertex::Vertex,
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

    pub fn visualize_vertex_chain(
        &self,
        vertices: &Vec<Vertex>,
        name: &String,
        vertex_radius: Option<f32>,
        vertex_color: Option<[u8; 4]>,
        edge_radius: Option<f32>,
        edge_color: Option<[u8; 4]>,
        draw_order: Option<f32>,
        close_chain: bool,
    ) -> Result<(), VisualizationError> {
        let vertex_radius = vertex_radius.unwrap_or(1.0);
        let vertex_color = vertex_color.unwrap_or(RandomColor::new().to_rgba_array());
        let draw_order = draw_order.unwrap_or(30.0);

        self.rec.log(
            format!("{name}/vertices"),
            &rerun::Points2D::new(vertices.iter().map(|v| (v.x as f32, v.y as f32)))
                .with_radii([vertex_radius])
                .with_colors([vertex_color])
                .with_draw_order(draw_order),
        )?;

        let edge_radius = edge_radius.unwrap_or(0.1);
        let edge_color = edge_color.unwrap_or(RandomColor::new().to_rgba_array());
        let mut edge_points = vertices
            .iter()
            .map(|v| (v.x as f32, v.y as f32))
            .collect_vec();
        if close_chain {
            edge_points.push(edge_points[0]);
        }
        self.rec.log(
            format!("{name}/edges"),
            &rerun::LineStrips2D::new([edge_points])
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

        let polygon_color = [132, 90, 109, 255];
        self.visualize_vertex_chain(
            &polygon.vertices().into_iter().cloned().collect(),
            &name,
            Some(0.5),
            Some(polygon_color),
            None,
            Some(polygon_color),
            None,
            true,
        )?;

        for (i, mesh) in rerun_meshes.iter().enumerate() {
            self.rec.log(format!("{name}/triangle_{i}"), mesh)?;
        }

        Ok(())
    }

    pub fn visualize_convex_hull(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        let polygon_color = [132, 90, 109, 255];
        let hull_color = [25, 100, 126, 255];

        let mut frame: i64 = 0;
        self.rec.set_time_sequence("frame", frame);

        self.visualize_nominal_polygon(&polygon, &name, polygon_color)?;

        self.increment_frame(&mut frame);
        let hull = QuickHull.convex_hull(polygon, &mut None);
        self.visualize_vertex_chain(
            &hull.vertices().into_iter().cloned().collect(),
            &format!("{name}/convex_hull"),
            Some(1.0),
            Some(hull_color),
            Some(0.3),
            Some(hull_color),
            Some(100.0),
            true,
        )?;

        Ok(())
    }

    pub fn visualize_convex_hull_graham_scan(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        let tracer = &mut Some(ConvexHullTracer::default());
        let _final_hull = GrahamScan.convex_hull(polygon, tracer);

        // TODO will ultimately want a config such that these could
        // be specified in some configurable or at the very least
        // more interpretable way? For now just hardcoding values
        // for color scheme I think looks decent
        let init_vertex_color = [255, 255, 255, 255];
        let polygon_color = [132, 90, 109, 255];
        let hull_color = [25, 100, 126, 255];
        let check_color = [242, 192, 53, 255];
        let valid_color = [52, 163, 82, 255];
        let invalid_color = [163, 0, 0, 255];

        let mut frame: i64 = 0;
        self.rec.set_time_sequence("frame", frame);

        self.visualize_nominal_polygon(&polygon, &name, polygon_color)?;

        // Show initial vertex establishing min angle order
        let id_0 = polygon.vertex_ids()[0];
        let v_0 = polygon.get_vertex(&id_0).unwrap();
        self.rec.log(
            format!("{name}/alg_init/init_vertex"),
            &rerun::Points2D::new([(v_0.x as f32, v_0.y as f32)])
                .with_radii([1.0])
                .with_colors([init_vertex_color])
                .with_draw_order(100.0),
        )?;

        let mut prev_step: Option<&ConvexHullTracerStep> = None;
        for (i, step) in tracer.as_ref().unwrap().steps.iter().enumerate() {
            if i == 0 {
                // Show initial edge of hull
                self.visualize_vertex_chain(
                    &polygon.get_vertices(step.hull_tail(2)),
                    &format!("{name}/hull_{i}"),
                    Some(0.8),
                    Some(hull_color),
                    Some(0.2),
                    Some(hull_color),
                    None,
                    false,
                )?;
            } else {
                self.increment_frame(&mut frame);

                // Show highlighted edge used for angle test
                let ids = prev_step
                    .expect("Prev step should exist i > 0")
                    .hull_tail(2);
                let v_origin = polygon.get_vertex(&ids[0]).unwrap();
                let v_head = polygon.get_vertex(&ids[1]).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/check_edge"),
                    &rerun::Arrows2D::from_vectors([(
                        (v_head.x - v_origin.x) as f32,
                        (v_head.y - v_origin.y) as f32,
                    )])
                    .with_origins([(v_origin.x as f32, v_origin.y as f32)])
                    .with_radii([0.3])
                    .with_colors([check_color])
                    .with_draw_order(100.0),
                )?;

                // Show next vertex used for angle test
                let n_id = step.next_vertex.expect("Next vertex should exist i > 0");
                let n_v = polygon.get_vertex(&n_id).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/next_vertex"),
                    &rerun::Points2D::new([(n_v.x as f32, n_v.y as f32)])
                        .with_radii([1.0])
                        .with_colors([check_color])
                        .with_draw_order(100.0),
                )?;

                self.rec.log(
                    format!("{name}/alg/next_vertex_marker"),
                    &rerun::LineStrips2D::new([[
                        (v_0.x as f32, v_0.y as f32),
                        (n_v.x as f32, n_v.y as f32),
                    ]])
                    .with_radii([0.1])
                    .with_colors([init_vertex_color]),
                )?;

                self.increment_frame(&mut frame);
                self.clear(format!("{name}/alg_{i}/check_edge"))?;
                self.clear(format!("{name}/alg_{i}/next_vertex"))?;

                let top_id = step.hull[step.hull.len() - 1];
                if n_id == top_id {
                    // Hull is fully repaired at this point, show final edge
                    // on stack connected to next vertex is a left turn (this
                    // will just be last 3 vertices in hull vertex chain
                    // since the next vertex was accepted to the hull)
                    self.visualize_vertex_chain(
                        &polygon.get_vertices(step.hull_tail(3)),
                        &format!("{name}/alg_{i}/valid"),
                        Some(1.0),
                        Some(valid_color),
                        Some(0.3),
                        Some(valid_color),
                        Some(100.0),
                        false,
                    )?;
                } else {
                    // Render final edge on stack to next vertex as invalid
                    // right turn
                    let mut ids = prev_step.expect("Prev step exists for i > 0").hull_tail(2);
                    ids.push(n_id);
                    self.visualize_vertex_chain(
                        &polygon.get_vertices(ids),
                        &format!("{name}/alg_{i}/invalid"),
                        Some(1.0),
                        Some(invalid_color),
                        Some(0.3),
                        Some(invalid_color),
                        Some(100.0),
                        false,
                    )?;
                }

                // Show computed hull for this step
                self.increment_frame(&mut frame);
                self.visualize_vertex_chain(
                    &polygon.get_vertices(step.hull.clone()),
                    &format!("{name}/hull_{i}"),
                    Some(0.8),
                    Some(hull_color),
                    Some(0.2),
                    Some(hull_color),
                    None,
                    true,
                )?;
            }
            prev_step = Some(step);

            self.clear_recursive(format!("{name}/alg_{i}"))?;
            // Clear out old hull visualizations
            if i > 0 {
                self.clear_recursive(format!("{name}/hull_{}", i - 1))?;
            }
        }

        self.increment_frame(&mut frame);
        self.visualize_final_hull(&polygon, tracer, &name, hull_color)?;

        Ok(())
    }

    pub fn visualize_convex_hull_incremental(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        let tracer = &mut Some(ConvexHullTracer::default());
        let _final_hull = Incremental.convex_hull(polygon, tracer);

        let mut frame: i64 = 0;
        self.rec.set_time_sequence("frame", frame);

        // TODO will ultimately want a config such that these could
        // be specified in some configurable or at the very least
        // more interpretable way? For now just hardcoding values
        // for color scheme I think looks decent
        let polygon_color = [132, 90, 109, 255];
        let hull_color = [25, 100, 126, 255];
        let next_vertex_color = [242, 192, 53, 255];
        let ut_color = [52, 163, 82, 255];
        let lt_color = [163, 0, 0, 255];

        self.visualize_nominal_polygon(&polygon, &name, polygon_color)?;

        // For each step will show upper/lower tangent vertex selection and
        // how they connect to the current hull, followed by the resulting
        // hull computed at that step
        for (i, step) in tracer.as_ref().unwrap().steps.iter().enumerate() {
            if i > 0 {
                self.increment_frame(&mut frame);

                let n_id = step.next_vertex.expect("Next vertex should exist i > 0");
                let n_v = polygon.get_vertex(&n_id).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/next_vertex"),
                    &rerun::Points2D::new([(n_v.x as f32, n_v.y as f32)])
                        .with_radii([1.0])
                        .with_colors([next_vertex_color])
                        .with_draw_order(100.0),
                )?;

                self.increment_frame(&mut frame);

                // Show upper/lower tangent vertices and their connection
                // to the current hull
                let ut_id = step
                    .upper_tangent_vertex
                    .expect("Upper tangent vertex should exist i > 0");
                self.visualize_vertex_chain(
                    &polygon.get_vertices(vec![ut_id, n_id]),
                    &format!("{name}/alg_{i}/upper_tangent"),
                    Some(1.0),
                    Some(ut_color),
                    Some(0.2),
                    Some(ut_color),
                    Some(90.0),
                    false,
                )?;

                let lt_id = step
                    .lower_tangent_vertex
                    .expect("Lower tangent vertex should exist i > 0");
                self.visualize_vertex_chain(
                    &polygon.get_vertices(vec![lt_id, n_id]),
                    &format!("{name}/alg_{i}/lower_tangent"),
                    Some(1.0),
                    Some(lt_color),
                    Some(0.2),
                    Some(lt_color),
                    Some(90.0),
                    false,
                )?;
            }

            // Show computed hull for this step
            self.increment_frame(&mut frame);
            self.visualize_vertex_chain(
                &polygon.get_vertices(step.hull.clone()),
                &format!("{name}/hull_{i}"),
                Some(0.8),
                Some(hull_color),
                Some(0.2),
                Some(hull_color),
                Some(50.0),
                true,
            )?;

            self.clear_recursive(format!("{name}/alg_{i}"))?;
            // Clear out old hull visualizations
            if i > 0 {
                self.clear_recursive(format!("{name}/hull_{}", i - 1))?;
            }
        }

        self.increment_frame(&mut frame);
        self.visualize_final_hull(&polygon, tracer, &name, hull_color)?;

        Ok(())
    }

    fn visualize_nominal_polygon(
        &self,
        polygon: &Polygon,
        name: &String,
        polygon_color: [u8; 4],
    ) -> Result<(), VisualizationError> {
        self.visualize_vertex_chain(
            &polygon.vertices().into_iter().cloned().collect(),
            &format!("{name}/polygon"),
            Some(0.5),
            Some(polygon_color),
            None,
            Some(polygon_color),
            Some(10.0),
            true,
        )
    }

    fn visualize_final_hull(
        &self,
        polygon: &Polygon,
        tracer: &mut Option<ConvexHullTracer>,
        name: &String,
        hull_color: [u8; 4],
    ) -> Result<(), VisualizationError> {
        let final_step = tracer.as_ref().unwrap().steps.last().unwrap();
        self.visualize_vertex_chain(
            &polygon.get_vertices(final_step.hull.clone()),
            &format!("{name}/hull_final"),
            Some(1.0),
            Some(hull_color),
            Some(0.3),
            Some(hull_color),
            Some(200.0),
            true,
        )
    }

    fn increment_frame(&self, frame: &mut i64) {
        *frame += 1;
        self.rec.set_time_sequence("frame", *frame);
    }

    fn clear(&self, name: String) -> Result<(), VisualizationError> {
        self.rec.log(name, &rerun::Clear::flat())?;
        Ok(())
    }

    fn clear_recursive(&self, name: String) -> Result<(), VisualizationError> {
        self.rec.log(name, &rerun::Clear::recursive())?;
        Ok(())
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
