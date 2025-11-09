use flexi_logger::{FileSpec, Logger};
use itertools::Itertools;
use random_color::RandomColor;

use crate::{
    alg_step::{parse_steps_from_logs, GrahamScanStep, IncrementalStep, StepParseError},
    convex_hull::{ConvexHullComputer, GrahamScan, Incremental, QuickHull},
    error::FileError,
    geometry::Geometry,
    polygon::Polygon,
    triangulation::{EarClipping, Triangulation, TriangulationComputer},
    vertex::{Vertex, VertexId},
};

#[derive(Debug)]
pub enum VisualizationError {
    File(FileError),
    FlexiLogger(flexi_logger::FlexiLoggerError),
    Rerun(rerun::RecordingStreamError),
    StepParse(StepParseError),
}

impl From<FileError> for VisualizationError {
    fn from(value: FileError) -> Self {
        VisualizationError::File(value)
    }
}

impl From<flexi_logger::FlexiLoggerError> for VisualizationError {
    fn from(value: flexi_logger::FlexiLoggerError) -> Self {
        VisualizationError::FlexiLogger(value)
    }
}

impl From<rerun::RecordingStreamError> for VisualizationError {
    fn from(value: rerun::RecordingStreamError) -> Self {
        VisualizationError::Rerun(value)
    }
}

impl From<StepParseError> for VisualizationError {
    fn from(value: StepParseError) -> Self {
        VisualizationError::StepParse(value)
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

    #[allow(clippy::too_many_arguments)]
    pub fn visualize_vertex_chain(
        &self,
        vertices: &[Vertex],
        name: &String,
        vertex_radius: Option<f32>,
        vertex_color: Option<[u8; 4]>,
        edge_radius: Option<f32>,
        edge_color: Option<[u8; 4]>,
        draw_order: Option<f32>,
        close_chain: bool,
        show_labels: bool,
    ) -> Result<(), VisualizationError> {
        let vertex_radius = vertex_radius.unwrap_or(1.0);
        let vertex_color = vertex_color.unwrap_or(RandomColor::new().to_rgba_array());
        let draw_order = draw_order.unwrap_or(30.0);

        let mut points = rerun::Points2D::new(vertices.iter().map(|v| (v.x as f32, v.y as f32)))
            .with_radii([vertex_radius])
            .with_colors([vertex_color])
            .with_draw_order(draw_order);

        if show_labels {
            let vertex_ids = vertices.iter().map(|v| v.id.to_string()).collect_vec();
            points = points.with_labels(vertex_ids);
        }

        self.rec.log(format!("{name}/vertices"), &points)?;

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
            &polygon.vertices().into_iter().cloned().collect_vec(),
            &name,
            Some(0.5),
            Some(polygon_color),
            None,
            Some(polygon_color),
            None,
            true,
            false,
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
        let hull_color = [25, 100, 126, 255];

        let mut frame: i64 = 0;
        self.rec.set_time_sequence("frame", frame);

        self.visualize_nominal_polygon(polygon, name)?;

        self.increment_frame(&mut frame);
        let hull = QuickHull.convex_hull(polygon);
        self.visualize_vertex_chain(
            &hull.vertices().into_iter().cloned().collect_vec(),
            &format!("{name}/convex_hull"),
            Some(1.0),
            Some(hull_color),
            Some(0.3),
            Some(hull_color),
            Some(100.0),
            true,
            false,
        )?;

        Ok(())
    }

    pub fn visualize_convex_hull_incremental(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        let file_spec = self.init_logger("visualizer_incremental".to_string())?;
        let final_hull = Incremental.convex_hull(polygon);
        let steps: Vec<IncrementalStep> =
            parse_steps_from_logs(file_spec.as_pathbuf(None)).unwrap();

        let mut frame: i64 = 0;
        self.rec.set_time_sequence("frame", frame);

        // TODO will ultimately want a config such that these could
        // be specified in some configurable or at the very least
        // more interpretable way? For now just hardcoding values
        // for color scheme I think looks decent
        let hull_color = [72, 125, 219, 255];
        let new_vertex_color = [242, 192, 53, 255];
        let ut_color = [212, 70, 110, 255];
        let lt_color = [242, 138, 27, 255];

        self.visualize_nominal_polygon(polygon, name)?;

        // For each step, show upper/lower tangent vertex selection and
        // how they connect to the current hull, followed by the
        // resulting hull computed at that step
        for (i, step) in steps.iter().enumerate() {
            if i > 0 {
                self.increment_frame(&mut frame);

                let new_id = step.new_id.expect("Should exist i > 0");
                let ut_id = step.ut_id.expect("Should exist i > 0");
                let lt_id = step.lt_id.expect("Should exist i > 0");

                let new_v = polygon.get_vertex(&new_id).unwrap();
                self.rec.log(
                    format!("{name}/alg_{i}/next_vertex"),
                    &rerun::Points2D::new([(new_v.x as f32, new_v.y as f32)])
                        .with_radii([1.0])
                        .with_colors([new_vertex_color])
                        .with_draw_order(100.0)
                        .with_labels([new_id.to_string()]),
                )?;
                self.rec.log(
                    "logs",
                    &rerun::TextLog::new(format!(
                        "Connecting vertex {new_id} to hull with upper/lower tangents"
                    ))
                    .with_level(rerun::TextLogLevel::DEBUG)
                    .with_color(new_vertex_color),
                )?;

                self.increment_frame(&mut frame);

                // Show upper/lower tangent vertices and their connection
                // to the current hull
                self.visualize_vertex_chain(
                    &polygon.get_vertices(vec![ut_id, new_id]),
                    &format!("{name}/alg_{i}/upper_tangent"),
                    Some(1.0),
                    Some(ut_color),
                    Some(0.2),
                    Some(ut_color),
                    Some(90.0),
                    false,
                    true,
                )?;
                self.rec.log(
                    "logs",
                    &rerun::TextLog::new(format!("Upper tangent: {new_id} -> {ut_id}"))
                        .with_level(rerun::TextLogLevel::DEBUG)
                        .with_color(ut_color),
                )?;
                self.visualize_vertex_chain(
                    &polygon.get_vertices(vec![lt_id, new_id]),
                    &format!("{name}/alg_{i}/lower_tangent"),
                    Some(1.0),
                    Some(lt_color),
                    Some(0.2),
                    Some(lt_color),
                    Some(90.0),
                    false,
                    true,
                )?;
                self.rec.log(
                    "logs",
                    &rerun::TextLog::new(format!("Lower tangent: {new_id} -> {lt_id}"))
                        .with_level(rerun::TextLogLevel::DEBUG)
                        .with_color(lt_color),
                )?;
            }

            // Show computed hull for this step
            self.increment_frame(&mut frame);
            self.visualize_vertex_chain(
                &polygon.get_vertices(step.hull_ids.clone()),
                &format!("{name}/hull_{i}"),
                Some(0.8),
                Some(hull_color),
                Some(0.2),
                Some(hull_color),
                Some(50.0),
                true,
                true,
            )?;
            self.rec.log(
                "logs",
                &rerun::TextLog::new(format!("Current hull stack: {:?}", step.hull_ids))
                    .with_level(rerun::TextLogLevel::DEBUG)
                    .with_color(hull_color),
            )?;

            self.clear_recursive(format!("{name}/alg_{i}"))?;
            // Clear out old hull visualizations
            if i > 0 {
                self.clear_recursive(format!("{name}/hull_{}", i - 1))?;
            }
        }

        self.increment_frame(&mut frame);
        self.visualize_final_hull(&final_hull, name)?;

        Ok(())
    }

    fn visualize_nominal_polygon(
        &self,
        polygon: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        // TODO this will be part of config
        let polygon_color = [132, 90, 109, 255];

        self.visualize_vertex_chain(
            &polygon.vertices().into_iter().cloned().collect_vec(),
            &format!("{name}/polygon"),
            Some(0.5),
            Some(polygon_color),
            None,
            Some(polygon_color),
            Some(10.0),
            true,
            false,
        )?;

        self.rec.log(
            "logs",
            &rerun::TextLog::new("Polygon to compute convex hull for")
                .with_level(rerun::TextLogLevel::DEBUG)
                .with_color(polygon_color),
        )?;

        Ok(())
    }

    fn visualize_final_hull(
        &self,
        final_hull: &Polygon,
        name: &String,
    ) -> Result<(), VisualizationError> {
        // TODO this will be part of config
        let hull_color = [72, 125, 219, 255];

        self.visualize_vertex_chain(
            &final_hull.get_vertices(final_hull.vertex_ids()),
            &format!("{name}/hull_final"),
            Some(1.0),
            Some(hull_color),
            Some(0.3),
            Some(hull_color),
            Some(200.0),
            true,
            true,
        )?;

        self.rec.log(
            "logs",
            &rerun::TextLog::new(format!("Final hull: {:?}", final_hull.vertex_ids()))
                .with_level(rerun::TextLogLevel::DEBUG)
                .with_color(hull_color),
        )?;

        Ok(())
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

    fn init_logger(&self, name: String) -> Result<FileSpec, VisualizationError> {
        let file_spec = FileSpec::default().directory("/tmp").basename(name);
        Logger::try_with_str("debug")?
            .log_to_file(file_spec.clone())
            .start()?;
        Ok(file_spec)
    }
}

pub struct GrahamScanVisualizer {
    rerun: RerunVisualizer,
}

impl GrahamScanVisualizer {
    pub fn new(name: String) -> Result<Self, VisualizationError> {
        let rerun = RerunVisualizer::new(name)?;
        Ok(Self { rerun })
    }

    pub fn execute_algorithm(
        &self,
        polygon: &Polygon,
    ) -> Result<(Polygon, Vec<GrahamScanStep>), VisualizationError> {
        let file_spec = self
            .rerun
            .init_logger("visualizer_graham_scan".to_string())?;
        let final_hull = GrahamScan.convex_hull(polygon);
        let steps: Vec<GrahamScanStep> = parse_steps_from_logs(file_spec.as_pathbuf(None))?;
        Ok((final_hull, steps))
    }

    pub fn visualize(&self, polygon: &Polygon, name: &String) -> Result<(), VisualizationError> {
        let (final_hull, steps) = self.execute_algorithm(polygon)?;

        // TODO will ultimately want a config such that these could
        // be specified in some configurable or at the very least
        // more interpretable way? For now just hardcoding values
        // for color scheme I think looks decent
        let init_vertex_color = [255, 255, 255, 255];
        let check_color = [242, 192, 53, 255];

        let mut frame: i64 = -1;
        self.rerun.increment_frame(&mut frame);
        self.rerun.visualize_nominal_polygon(polygon, name)?;
        self.rerun.increment_frame(&mut frame);

        // Show initial vertex establishing min angle order
        let id_0 = polygon.vertex_ids()[0];
        let v_0 = polygon.get_vertex(&id_0).unwrap();
        self.rerun.rec.log(
            format!("{name}/alg_init/init_vertex"),
            &rerun::Points2D::new([(v_0.x as f32, v_0.y as f32)])
                .with_radii([1.0])
                .with_colors([init_vertex_color])
                .with_draw_order(100.0)
                .with_labels([id_0.to_string()]),
        )?;

        let mut prev_step: Option<&GrahamScanStep> = None;
        for (i, step) in steps.iter().enumerate() {
            if i == 0 {
                // Show initial edge of hull
                self.visualize_hull(step.hull_ids.clone(), &polygon, &name, i, false)?;
            } else {
                self.rerun.increment_frame(&mut frame);

                // Show highlighted edge used for angle test
                let ids = prev_step
                    .expect("Prev step should exist i > 0")
                    .hull_tail(2);
                let v_origin = polygon.get_vertex(&ids[0]).unwrap();
                let v_head = polygon.get_vertex(&ids[1]).unwrap();
                self.rerun.rec.log(
                    format!("{name}/alg_{i}/check_edge"),
                    &rerun::Arrows2D::from_vectors([(
                        (v_head.x - v_origin.x) as f32,
                        (v_head.y - v_origin.y) as f32,
                    )])
                    .with_origins([(v_origin.x as f32, v_origin.y as f32)])
                    .with_radii([0.3])
                    .with_colors([check_color])
                    .with_draw_order(100.0)
                    .with_labels([format!("{} -> {}", v_origin.id, v_head.id)]),
                )?;

                // Show next vertex used for angle test
                let new_id = step.new_id.expect("Should exist i > 0");
                let new_v = polygon.get_vertex(&new_id).unwrap();
                self.rerun.rec.log(
                    format!("{name}/alg_{i}/next_vertex"),
                    &rerun::Points2D::new([(new_v.x as f32, new_v.y as f32)])
                        .with_radii([1.0])
                        .with_colors([check_color])
                        .with_draw_order(100.0)
                        .with_labels([new_id.to_string()]),
                )?;

                self.rerun.rec.log(
                    "logs",
                    &rerun::TextLog::new(format!(
                        "Checking angle between vector {} -> {} and vertex {}",
                        v_origin.id, v_head.id, new_id
                    ))
                    .with_level(rerun::TextLogLevel::DEBUG)
                    .with_color(check_color),
                )?;

                self.rerun.rec.log(
                    format!("{name}/alg/next_vertex_marker"),
                    &rerun::LineStrips2D::new([[
                        (v_0.x as f32, v_0.y as f32),
                        (new_v.x as f32, new_v.y as f32),
                    ]])
                    .with_radii([0.1])
                    .with_colors([init_vertex_color]),
                )?;

                self.rerun.increment_frame(&mut frame);
                self.rerun.clear(format!("{name}/alg_{i}/check_edge"))?;
                self.rerun.clear(format!("{name}/alg_{i}/next_vertex"))?;

                if new_id == step.hull_top() {
                    // Hull is fully repaired at this point, show final edge
                    // on stack connected to next vertex is a left turn (this
                    // will just be last 3 vertices in hull vertex chain
                    // since the next vertex was accepted to the hull)
                    self.visualize_check_result(step.hull_tail(3), polygon, name, i, true)?;
                    self.rerun.increment_frame(&mut frame);
                } else {
                    // Render final edge on stack to next vertex as invalid
                    // right turn
                    let mut ids = prev_step.expect("Prev step exists for i > 0").hull_tail(2);
                    ids.push(new_id);
                    self.visualize_check_result(ids.clone(), polygon, name, i, false)?;
                    self.rerun.increment_frame(&mut frame);

                    // Keep visualization of vertex being checked for next iter
                    self.rerun.rec.log(
                        format!("{name}/alg_{}/next_vertex", i + 1),
                        &rerun::Points2D::new([(new_v.x as f32, new_v.y as f32)])
                            .with_radii([1.0])
                            .with_colors([check_color])
                            .with_draw_order(100.0)
                            .with_labels([new_id.to_string()]),
                    )?;
                }

                // Show computed hull for this step
                self.visualize_hull(step.hull_ids.clone(), &polygon, &name, i, false)?;
            }
            prev_step = Some(step);

            self.rerun.clear_recursive(format!("{name}/alg_{i}"))?;
            // Clear out old hull visualizations
            if i > 0 {
                self.rerun
                    .clear_recursive(format!("{name}/hull_{}", i - 1))?;
            }
        }

        self.rerun.increment_frame(&mut frame);
        self.rerun.visualize_final_hull(&final_hull, name)?;

        Ok(())
    }

    fn visualize_hull(
        &self,
        hull_ids: Vec<VertexId>,
        polygon: &Polygon,
        name: &String,
        step_index: usize,
        initial: bool,
    ) -> Result<(), VisualizationError> {
        // TODO this will be part of config
        let hull_color = [72, 125, 219, 255];

        self.rerun.visualize_vertex_chain(
            &polygon.get_vertices(hull_ids.clone()),
            &format!("{name}/hull_{step_index}"),
            Some(0.8),
            Some(hull_color),
            Some(0.2),
            Some(hull_color),
            None,
            !initial,
            true,
        )?;

        if initial {
            self.rerun.rec.log(
                "logs",
                &rerun::TextLog::new(format!(
                    "Initialized with hull edge {} -> {}",
                    hull_ids[0], hull_ids[1]
                ))
                .with_level(rerun::TextLogLevel::DEBUG)
                .with_color(hull_color),
            )?;
        } else {
            self.rerun.rec.log(
                "logs",
                &rerun::TextLog::new(format!("Current hull stack: {:?}", hull_ids))
                    .with_level(rerun::TextLogLevel::DEBUG)
                    .with_color(hull_color),
            )?;
        }
        Ok(())
    }

    fn visualize_check_result(
        &self,
        vertex_ids: Vec<VertexId>,
        polygon: &Polygon,
        name: &String,
        step_index: usize,
        valid: bool,
    ) -> Result<(), VisualizationError> {
        // TODO this will be part of config
        let valid_color = [52, 163, 82, 255];
        let invalid_color = [235, 64, 52, 255];

        let color = match valid {
            true => valid_color,
            false => invalid_color,
        };

        self.rerun.visualize_vertex_chain(
            &polygon.get_vertices(vertex_ids.clone()),
            &format!("{name}/alg_{step_index}/valid"),
            Some(1.0),
            Some(color),
            Some(0.3),
            Some(color),
            Some(100.0),
            false,
            true,
        )?;

        let msg = match valid {
            true => format!("Pushing valid vertex to hull stack: {}", vertex_ids[0]),
            false => format!("Popping invalid vertex from hull stack: {}", vertex_ids[1]),
        };

        self.rerun.rec.log(
            "logs",
            &rerun::TextLog::new(msg)
                .with_level(rerun::TextLogLevel::DEBUG)
                .with_color(color),
        )?;

        Ok(())
    }
}
