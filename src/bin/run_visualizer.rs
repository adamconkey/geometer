use clap::Parser;

use geometer::util::load_polygon;
use geometer::visualizer::RerunVisualizer;


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
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let visualizer = RerunVisualizer::new("Geometer".to_string());

    // TODO have load function return Result
    let polygon = load_polygon(&args.polygon, &args.folder);
    let name = format!("{}/{}", args.polygon, args.folder);

    visualizer.visualize_triangulation(&polygon, &name);
    
    Ok(())
}