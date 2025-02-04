use clap::Parser;
use random_color::RandomColor;

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
    let tri_points = triangulation.to_points();

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

    let mesh_1 = rerun::Mesh3D::new(
        [
            [tri_points[0].0.x, tri_points[0].0.y, 0.0],
            [tri_points[0].1.x, tri_points[0].1.y, 0.0],
            [tri_points[0].2.x, tri_points[0].2.y, 0.0],
        ]
    );
    
    let mesh_2 = rerun::Mesh3D::new(
        [
            [tri_points[1].0.x, tri_points[1].0.y, 0.0],
            [tri_points[1].1.x, tri_points[1].1.y, 0.0],
            [tri_points[1].2.x, tri_points[1].2.y, 0.0],
        ]
    );
    

    let rec = rerun::RecordingStreamBuilder::new("rerun_example_minimal").connect_tcp()?;

    let mut color = RandomColor::new();
    let rgb = color.to_rgb_array();

    let mut other_color = RandomColor::new();
    let other_rgb = other_color.to_rgb_array();

    rec.log(
        "triangulation/triangle_1",
        &mesh_1
            .with_vertex_normals([[0.0, 0.0, 1.0]])
            .with_vertex_colors([rgb, rgb, rgb])
    )?;


    rec.log(
        "triangulation/triangle_2",
        &mesh_2
            .with_vertex_normals([[0.0, 0.0, 1.0]])
            .with_vertex_colors([other_rgb, other_rgb, other_rgb])
    )?;

    // let polygon_points = polygon.sorted_points().into_iter().map(|p| (p.x, p.y));
    // let mut edge_points: Vec<(f32, f32)> = polygon_points.clone().collect();
    // edge_points.push(edge_points[0]);
    // let points = rerun::Points2D::new(polygon_points);
    // let edges = rerun::LineStrips2D::new([edge_points]);

    // rec.log(
    //     format!("{}/{}/vertices", args.folder, args.polygon),
    //     &points
    //         .with_radii([0.5]),
    // )?;

    // rec.log(
    //     format!("{}/{}/edges", args.folder, args.polygon),
    //     &edges
    // )?;

    Ok(())
}