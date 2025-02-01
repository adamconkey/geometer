use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use computational_geometry::util::polygon_map_by_num_vertices;


fn benchmark_interior_points(c: &mut Criterion) {
    let polygon_map = polygon_map_by_num_vertices();
    let mut group = c.benchmark_group("Interior Points");
    group.sample_size(10);

    for (name, polygon) in polygon_map.iter() {
        group.bench_with_input(
            BenchmarkId::new("interior_points_from_extreme_edges", name),
            polygon, 
            |b, polygon| b.iter(|| polygon.interior_points_from_extreme_edges())
        );
        group.bench_with_input(
            BenchmarkId::new("interior_points_from_triangle_checks", name), 
            polygon,
            |b, polygon| b.iter(|| polygon.interior_points_from_triangle_checks())
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark_interior_points);
criterion_main!(benches);