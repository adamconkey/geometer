use clap::Parser;

use geometer::util::load_polygon;


/// Visualize polygons and algorithms using Rerun.io``
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder containing polygon file
    #[arg(short, long, default_value = "custom")]
    folder: String,

    /// Polygon name to visualize (without the .json extension)
    #[arg(short, long, default_value = "polygon_2")]
    polygon: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // TODO need to have this return result and handle errors gracefully
    let polygon = load_polygon(args.polygon.as_str(), args.folder.as_str());
    let triangulation = polygon.triangulation();
    let rerun_meshes = triangulation.to_rerun_meshes();

    // TODO this is obviously very hacked but illustrates the way forward
    // I think. I can visualize each triangle individually as a 3D mesh
    // (not sure if that will have implications later if I have many many
    // meshes in the scene but lets assume it's fine for now). Just need
    // to convert each to a rerun mesh with a random color, can add some
    // helpers to do this and then just map iter-map the triangulation to
    // the meshes (could consider adding that directly to the struct?).
    // Then just iterate and publish them all. I think if you visualize
    // this together with the polygon itself (edges and vertices), then 
    // you can make a nice visualization of the order in which it generates
    // the triangles. Will need to store that somehow along with the
    // triangulation result, so would either want to order the output
    // by its insertion or just store that as it goes. Then just need
    // to figure out how to correctly time things to send to rerun so
    // it visualizes the algorithm in a nice way.    

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