use clap::{Parser, ValueEnum};

use geometer::{
    convex_hull_visualizer::{RerunVisualizer, VisualizationError},
    util::load_polygon,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Visualization {
    ConvexHull,
    ConvexHullGrahamScan,
    ConvexHullIncremental,
    Triangulation,
}

/// Visualize polygons and algorithms using Rerun.io
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

fn main() -> Result<(), VisualizationError> {
    let args = Args::parse();
    let visualizer = RerunVisualizer::new("Geometer".to_string())?;

    let polygon = load_polygon(&args.polygon, &args.folder)?;
    let name = format!("{}/{}", args.polygon, args.folder);

    match args.visualization {
        Visualization::ConvexHull => visualizer.visualize_convex_hull(&polygon, &name)?,
        Visualization::ConvexHullGrahamScan => {
            // TODO this is temporary just to compile for now,
            // will ultimately create separate visualizers for
            // each one and run them.
            visualizer.visualize_convex_hull_incremental(&polygon, &name)?
        }
        Visualization::ConvexHullIncremental => {
            visualizer.visualize_convex_hull_incremental(&polygon, &name)?
        }
        Visualization::Triangulation => visualizer.visualize_triangulation(&polygon, &name)?,
    };

    Ok(())
}
