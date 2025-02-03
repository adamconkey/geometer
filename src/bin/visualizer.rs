use geometer::util::load_polygon;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_minimal").connect_tcp()?;

    let polygon = load_polygon("polygon_2", "custom");

    let polygon_points = polygon.sorted_points().into_iter().map(|p| (p.x, p.y));
    let mut edge_points: Vec<(f32, f32)> = polygon_points.clone().collect();
    edge_points.push(edge_points[0]);
    let points = rerun::Points2D::new(polygon_points);
    let edges = rerun::LineStrips2D::new([edge_points]);

    rec.log(
        "polygon_2/vertices",
        &points
            .with_radii([0.5]),
    )?;

    rec.log(
        "polygon_2/edges",
        &edges
    )?;

    Ok(())
}