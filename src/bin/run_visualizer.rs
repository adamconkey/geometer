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


pub struct RerunVisualizer {
    rec: rerun::RecordingStream,
}


impl RerunVisualizer {
    pub fn new(name: String) -> Self {
        // TODO don't unwrap
        let rec = rerun::RecordingStreamBuilder::new(name).connect_tcp().unwrap();
        RerunVisualizer { rec }
    }

    pub fn visualize_polygon(&self, polygon: &Polygon, name: &String) {
        self.rec.log(
            format!("{}/vertices", name),
            &self.polygon_to_rerun_points(&polygon),
        ).unwrap();  // TODO don't unwrap

        self.rec.log(
            format!("{}/edges", name),
            &self.polygon_to_rerun_edges(&polygon)
                .with_colors([(3, 144, 252)])
        ).unwrap();  // TODO don't unwrap
    }

    // TODO need to have this return Result and handle errors gracefully
    pub fn visualize_triangulation(&self, polygon: &Polygon, name: &String) {
        let name = format!("{name}/triangulation");
        let triangulation = polygon.triangulation();
        let rerun_meshes = self.triangulation_to_rerun_meshes(&triangulation);

        self.visualize_polygon(polygon, &name);
        
        for (i, mesh) in rerun_meshes.iter().enumerate() {
            self.rec.log(
                format!("{}/triangle_{}", &name, i),
                mesh
            ).unwrap();  // TODO don't unwrap
        }
    }

    // TODO have this return Result
    pub fn visualize_extreme_points(&self, polygon: &Polygon, name: &String) {
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
                .map(|p| (p.x, p.y, 0.0))
        );

        self.rec.log(
            format!("{}/extreme_points", name),
            &rerun_points
                .with_radii([5.0])
                .with_colors([(252, 207, 3)]),
        ).unwrap();  // TODO don't unwrap
    }

    fn polygon_to_rerun_points(&self, polygon: &Polygon) -> rerun::Points3D {
        rerun::Points3D::new(
            polygon.sorted_points()
                .into_iter()
                .map(|p| (p.x, p.y, 0.0))
        )
    }
    
    fn polygon_to_rerun_edges(&self, polygon: &Polygon) -> rerun::LineStrips3D {
        let mut edge_points: Vec<_> = polygon.sorted_points()
            .into_iter()
            .map(|p| (p.x, p.y, 0.0))
            .collect();
        edge_points.push(edge_points[0]);
        rerun::LineStrips3D::new([edge_points])
    }
    
    fn triangulation_to_rerun_meshes(&self, triangulation: &Triangulation) -> Vec<rerun::Mesh3D> {
        let mut meshes = Vec::new();
        for (p1, p2, p3) in triangulation.to_points().iter() { 
            let color = RandomColor::new().to_rgb_array();
            let points = [[p1.x, p1.y, 0.0], [p2.x, p2.y, 0.0], [p3.x, p3.y, 0.0]]; 
            let mesh = rerun::Mesh3D::new(points)
                .with_vertex_colors([color, color, color]);
            meshes.push(mesh);
        }
        meshes
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
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