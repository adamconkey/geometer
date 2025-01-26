use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};
use walkdir::WalkDir;

use computational_geometry::polygon::Polygon;


fn polygon_map() -> HashMap<usize, Polygon> {
    let mut map = HashMap::new();
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("polygons");
    let paths = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        // TODO this is a bit hacky, it's meant to account for files ending
        // in .meta.json but it won't be perfect, should figure out a nicer
        // way to ensure it picks up exactly the intended files
        .filter(|p| p.is_file())
        .filter(|p| p.extension() == Some(OsStr::new("json")))
        .filter(|p| p.with_extension("").extension() != Some(OsStr::new("meta")));

    for path in paths {
        println!("{:?}", path);
        let p = Polygon::from_json(path);
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