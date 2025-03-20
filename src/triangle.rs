use std::{cell::OnceCell, collections::HashSet};

use crate::{
    geometry::Geometry,
    line_segment::LineSegment,
    vertex::{Vertex, VertexId},
};

pub struct Triangle<'a> {
    pub v1: &'a Vertex,
    pub v2: &'a Vertex,
    pub v3: &'a Vertex,
    area: OnceCell<f64>,
}

impl Geometry for Triangle<'_> {
    fn vertices(&self) -> Vec<&Vertex> {
        vec![self.v1, self.v2, self.v3]
    }

    fn edges(&self) -> HashSet<(VertexId, VertexId)> {
        let mut edges = HashSet::new();
        edges.insert((self.v1.id, self.v2.id));
        edges.insert((self.v2.id, self.v3.id));
        edges.insert((self.v3.id, self.v1.id));
        edges
    }

    fn get_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        todo!()
    }

    fn get_prev_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        todo!()
    }

    fn get_next_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        todo!()
    }
}

impl<'a> Triangle<'a> {
    pub fn from_vertices(v1: &'a Vertex, v2: &'a Vertex, v3: &'a Vertex) -> Self {
        let area = OnceCell::new();
        Self { v1, v2, v3, area }
    }

    pub fn to_line_segments(&self) -> Vec<LineSegment> {
        let ls1 = LineSegment::from_vertices(self.v1, self.v2);
        let ls2 = LineSegment::from_vertices(self.v2, self.v3);
        let ls3 = LineSegment::from_vertices(self.v3, self.v1);
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
        let triangle = Triangle::from_vertices(&a, &b, &c);
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
            Triangle::from_vertices(&a, &c, &b),
            Triangle::from_vertices(&c, &b, &a),
            Triangle::from_vertices(&b, &a, &c),
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
            Triangle::from_vertices(&a, &b, &c),
            Triangle::from_vertices(&b, &c, &a),
            Triangle::from_vertices(&c, &a, &b),
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
            let triangle = Triangle::from_vertices(v0, v1, v2);

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
