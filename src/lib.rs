#[cfg(test)]
// This is currently the strictest I could be across ALL
// tests that use float asserts. Most tests satisfy a
// lot more precision so not sure if this is the best
// way to go about this, but for now this shows the
// empirical precision limit on the entire test suite
const F32_ASSERT_PRECISION: f32 = 1e-4f32;

pub mod bounding_box;
pub mod line_segment;
pub mod point;
pub mod polygon;
pub mod triangle;
pub mod triangulation;
pub mod util;
pub mod vertex;
pub mod vertex_map;
pub mod visualizer;