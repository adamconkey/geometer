use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use computational_geometry::util::polygon_map_by_num_vertices;


fn benchmark_extreme_points(c: &mut Criterion) {
    let polygon_map = polygon_map_by_num_vertices(200usize);
    let mut group = c.benchmark_group("Extreme Points");
    group.sample_size(10);

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