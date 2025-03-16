use std::cell::OnceCell;

use crate::{line_segment::LineSegment, vertex::Vertex};

pub struct Triangle {
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
    area: OnceCell<f64>,
}

impl Triangle {
    pub fn from_vertices(v1: Vertex, v2: Vertex, v3: Vertex) -> Triangle {
        Triangle {
            v1,
            v2,
            v3,
            area: OnceCell::new(),
        }
    }

    pub fn to_line_segments(&self) -> Vec<LineSegment> {
        let ls1 = LineSegment::from_vertices(self.v1.clone(), self.v2.clone());
        let ls2 = LineSegment::from_vertices(self.v2.clone(), self.v3.clone());
        let ls3 = LineSegment::from_vertices(self.v3.clone(), self.v1.clone());
        vec![ls1, ls2, ls3]
    }

    pub fn area(&self) -> f64 {
        *self.area.get_or_init(|| {
            let t1 = self.v2.x - self.v1.x;
            let t2 = self.v3.y - self.v1.y;
            let t3 = self.v3.x - self.v1.x;
            let t4 = self.v2.y - self.v1.y;
            0.5 * ((t1 * t2) - (t3 * t4))
        })
    }

    pub fn has_collinear_points(&self) -> bool {
        self.area() == 0.0
    }

    pub fn contains(&self, v: &Vertex) -> bool {
        if self.has_collinear_points() {
            return false;
        }
        for ls in self.to_line_segments() {
            if !v.left_on(&ls) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vertex::VertexId;
    use itertools::Itertools;

    #[test]
    fn test_area_right_triangle() {
        let a = Vertex::new(VertexId::from(0u32), 0.0, 0.0);
        let b = Vertex::new(VertexId::from(1u32), 3.0, 0.0);
        let c = Vertex::new(VertexId::from(2u32), 0.0, 4.0);
        let triangle = Triangle::from_vertices(a, b, c);
        let area = triangle.area();
        assert_eq!(area, 6.0);
    }

    // TODO want some better unit tests for the triangle area

    #[test]
    fn test_area_clockwise() {
        let a = Vertex::new(VertexId::from(0u32), 0.0, 0.0);
        let b = Vertex::new(VertexId::from(1u32), 4.0, 3.0);
        let c = Vertex::new(VertexId::from(2u32), 1.0, 3.0);

        let cw = vec![
            Triangle::from_vertices(a.clone(), c.clone(), b.clone()),
            Triangle::from_vertices(c.clone(), b.clone(), a.clone()),
            Triangle::from_vertices(b, a, c),
        ];
        for triangle in cw {
            assert!(triangle.area() < 0.0);
        }
    }

    #[test]
    fn test_area_counter_clockwise() {
        let a = Vertex::new(VertexId::from(0u32), 0.0, 0.0);
        let b = Vertex::new(VertexId::from(1u32), 4.0, 3.0);
        let c = Vertex::new(VertexId::from(2u32), 1.0, 3.0);

        let ccw = vec![
            Triangle::from_vertices(a.clone(), b.clone(), c.clone()),
            Triangle::from_vertices(b.clone(), c.clone(), a.clone()),
            Triangle::from_vertices(c, a, b),
        ];
        for triangle in ccw {
            assert!(triangle.area() > 0.0);
        }
    }

    #[test]
    fn test_area_collinear() {
        let a = Vertex::new(VertexId::from(0u32), 0.0, 0.0);
        let b = Vertex::new(VertexId::from(1u32), 4.0, 3.0);
        let c = Vertex::new(VertexId::from(2u32), 1.0, 3.0);

        // This is choice with replacement over a 3-tuple, so there are
        // 3 * 3 * 3 = 27 total options and this generates all of them.
        let all_combos = std::iter::repeat(vec![&a, &b, &c].into_iter())
            .take(3)
            .multi_cartesian_product();

        for vertices in all_combos {
            let v0 = vertices[0];
            let v1 = vertices[1];
            let v2 = vertices[2];
            let triangle = Triangle::from_vertices(v0.clone(), v1.clone(), v2.clone());

            if v0 == v1 || v0 == v2 || v1 == v2 {
                // If there's duplicate vertices, they should be detected
                // as collinear (zero area)
                assert!(triangle.has_collinear_points());
            } else {
                // If all vertices are unique then they're either clockwise
                // (negative area) or counter-clockwise (positive area)
                assert!(!triangle.has_collinear_points());
            }
        }
    }
}
