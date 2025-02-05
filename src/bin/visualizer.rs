use clap::Parser;

use geometer::util::load_polygon;


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

    // TODO need to have this return result and handle errors gracefully
    let polygon = load_polygon(args.polygon.as_str(), args.folder.as_str());
    let triangulation = polygon.triangulation();
    let rerun_meshes = triangulation.to_rerun_meshes();

    let rec = rerun::RecordingStreamBuilder::new("rerun_example_minimal").connect_tcp()?;

    rec.log(
        format!("{}/{}/vertices", args.folder, args.polygon),
        &polygon.to_rerun_points()
            .with_radii([0.3]),
    )?;

    rec.log(
        format!("{}/{}/edges", args.folder, args.polygon),
        &polygon.to_rerun_edges()
    )?;

    for (i, mesh) in rerun_meshes.iter().enumerate() {
        rec.log(
            format!("{}/{}/triangle_{}", args.folder, args.polygon, i),
            mesh
        )?;
    }

    Ok(())
}