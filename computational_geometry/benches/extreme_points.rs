use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{collections::HashMap, path::PathBuf};

use computational_geometry::polygon::Polygon;


fn load_polygon(name: &str, folder: &str) -> Polygon {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("polygons");
    path.push(folder);
    path.push(format!("{}.json", name));
    Polygon::from_json(path)
}

fn polygon_map() -> HashMap<usize, Polygon> {
    let mut map = HashMap::new();
    for name in ["polygon_1", "polygon_2", "square_4x4", "right_triangle"].iter() {
        let p = load_polygon(name, "custom");
        map.insert(p.num_vertices(), p);
    }
    map
}

fn benchmark_extreme_points(c: &mut Criterion) {
    let polygon_map = polygon_map();
    let mut group = c.benchmark_group("Extreme Points");

    for (name, polygon) in polygon_map.iter() {
        group.bench_with_input(
            BenchmarkId::new("extreme_points_from_edges", name),
            polygon, 
            |b, polygon| b.iter(|| polygon.extreme_points_from_extreme_edges())
        );
        group.bench_with_input(
            BenchmarkId::new("extreme_points_from_interior_points", name), 
            polygon,
            |b, polygon| b.iter(|| polygon.extreme_points_from_interior_points())
        );
    }
    group.finish();
}


criterion_group!(benches, benchmark_extreme_points);
criterion_main!(benches);