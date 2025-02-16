use clap::{Parser, ValueEnum};
use random_color::RandomColor;

use geometer::polygon::Polygon;
use geometer::triangulation::Triangulation;
use geometer::util::load_polygon;


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Visualization {
    Triangulation,
    ExtremePoints,
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
    Rerun(rerun::RecordingStreamError),
}


pub struct RerunVisualizer {
    rec: rerun::RecordingStream,
}


impl RerunVisualizer {
    pub fn new(name: String) -> Result<Self, VisualizationError> {
        let rec = rerun::RecordingStreamBuilder::new(name).connect_tcp()?;
        Ok(RerunVisualizer { rec })
    }

    pub fn visualize_polygon(&self, polygon: &Polygon, name: &String) -> Result<(), VisualizationError> {
        self.rec.log(
            format!("{}/vertices", name),
            &self.polygon_to_rerun_points(polygon),
        )?;

        self.rec.log(
            format!("{}/edges", name),
            &self.polygon_to_rerun_edges(polygon)
                .with_colors([(3, 144, 252)])
        )?;
        
        Ok(())
    }

    pub fn visualize_triangulation(&self, polygon: &Polygon, name: &String) -> Result<(), VisualizationError> {
        let name = format!("{name}/triangulation");
        let triangulation = polygon.triangulation();
        let rerun_meshes = self.triangulation_to_rerun_meshes(&triangulation);

        self.visualize_polygon(polygon, &name);
        
        for (i, mesh) in rerun_meshes.iter().enumerate() {
            self.rec.log(
                format!("{}/triangle_{}", &name, i),
                mesh
            )?;
        }

        Ok(())
    }

    pub fn visualize_extreme_points(&self, polygon: &Polygon, name: &String) -> Result<(), VisualizationError> {
        let name = format!("{name}/extreme_points");
        
        self.visualize_polygon(polygon, &name);
        
        let extreme_points: Vec<_> = polygon
            .extreme_points()
            .iter()
            .map(|id| polygon.get_point(id))
            .collect();

        let rerun_points = rerun::Points3D::new(
            extreme_points
                .into_iter()
                .map(|p| (p.x as f32, p.y as f32, 0.0))
        );

        self.rec.log(
            format!("{}/extreme_points", name),
            &rerun_points
                .with_radii([5.0])
                .with_colors([(252, 207, 3)]),
        )?;

        Ok(())
    }

    fn polygon_to_rerun_points(&self, polygon: &Polygon) -> rerun::Points3D {
        rerun::Points3D::new(
            polygon.sorted_points()
                .into_iter()
                .map(|p| (p.x as f32, p.y as f32, 0.0))
        )
    }
    
    fn polygon_to_rerun_edges(&self, polygon: &Polygon) -> rerun::LineStrips3D {
        let mut edge_points: Vec<_> = polygon.sorted_points()
            .into_iter()
            .map(|p| (p.x as f32, p.y as f32, 0.0))
            .collect();
        edge_points.push(edge_points[0]);
        rerun::LineStrips3D::new([edge_points])
    }
    
    fn triangulation_to_rerun_meshes(&self, triangulation: &Triangulation) -> Vec<rerun::Mesh3D> {
        let mut meshes = Vec::new();
        for (p1, p2, p3) in triangulation.to_points().iter() { 
            let color = RandomColor::new().to_rgb_array();
            let points = [
                [p1.x as f32, p1.y as f32, 0.0], 
                [p2.x as f32, p2.y as f32, 0.0], 
                [p3.x as f32, p3.y as f32, 0.0]
            ]; 
            let mesh = rerun::Mesh3D::new(points)
                .with_vertex_colors([color, color, color]);
            meshes.push(mesh);
        }
        meshes
    }
}


fn main() -> Result<(), VisualizationError> {
    let args = Args::parse();
    let visualizer = RerunVisualizer::new("Geometer".to_string());

    // TODO have load function return Result
    let polygon = load_polygon(&args.polygon, &args.folder);
    let name = format!("{}/{}", args.polygon, args.folder);

    match args.visualization {
        Visualization::ExtremePoints => visualizer.visualize_extreme_points(&polygon, &name),
        Visualization::Triangulation => visualizer.visualize_triangulation(&polygon, &name),
    };
    
    Ok(())
}