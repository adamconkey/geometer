use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};
use walkdir::WalkDir;

use computational_geometry::polygon::Polygon;


fn polygon_map_by_num_vertices() -> HashMap<usize, Polygon> {
    let mut map = HashMap::new();
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("polygons");
    let paths = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file())
        .filter(|p| p.extension() == Some(OsStr::new("json")))
        // Remove .meta.json files
        .filter(|p| p.with_extension("").extension() != Some(OsStr::new("meta")));
    for path in paths.sorted() {
        let p = Polygon::from_json(path);
        map.insert(p.num_vertices(), p);
    }
    map
}

fn benchmark_extreme_points(c: &mut Criterion) {
    let polygon_map = polygon_map_by_num_vertices();
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