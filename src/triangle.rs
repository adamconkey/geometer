use std::cell::OnceCell;

use crate::{
    line_segment::LineSegment,
    point::Point,
    vertex::{Vertex, VertexId},
};

pub struct Triangle {
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
    area: OnceCell<f64>,
}

impl Triangle {
    pub fn from_points(p1: Point, p2: Point, p3: Point) -> Triangle {
        let id1 = VertexId::from(0u32);
        let id2 = VertexId::from(1u32);
        let id3 = VertexId::from(2u32);
        let v1 = Vertex::new(p1, id1, id3, id2);
        let v2 = Vertex::new(p2, id2, id1, id3);
        let v3 = Vertex::new(p3, id3, id2, id1);
        Triangle::from_vertices(v1, v2, v3)
    }

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
            let t1 = self.v2.coords.x - self.v1.coords.x;
            let t2 = self.v3.coords.y - self.v1.coords.y;
            let t3 = self.v3.coords.x - self.v1.coords.x;
            let t4 = self.v2.coords.y - self.v1.coords.y;
            0.5 * ((t1 * t2) - (t3 * t4))
        })
    }

    pub fn has_collinear_points(&self) -> bool {
        self.area() == 0.0
    }

    pub fn contains(&self, p: Point) -> bool {
        if self.has_collinear_points() {
            return false;
        }
        for ls in self.to_line_segments() {
            if !p.left_on(&ls) {
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
    fn test_from_vertices() {
        let id1 = VertexId::from(1u32);
        let id2 = VertexId::from(2u32);
        let id3 = VertexId::from(3u32);
        let v1 = Vertex::new(Point::new(0.0, 0.0), id1, id3, id2);
        let v2 = Vertex::new(Point::new(3.0, 0.0), id2, id1, id3);
        let v3 = Vertex::new(Point::new(0.0, 4.0), id3, id2, id1);
        let triangle = Triangle::from_vertices(v1, v2, v3);
        assert_eq!(Point::new(0.0, 0.0), triangle.v1.coords);
        assert_eq!(Point::new(3.0, 0.0), triangle.v2.coords);
        assert_eq!(Point::new(0.0, 4.0), triangle.v3.coords);
    }

    #[test]
    fn test_area_right_triangle() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 0.0);
        let c = Point::new(0.0, 4.0);
        let triangle = Triangle::from_points(a, b, c);
        let area = triangle.area();
        assert_eq!(area, 6.0);
    }

    // TODO want some better unit tests for the triangle area

    #[test]
    fn test_area_clockwise() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(4.0, 3.0);
        let c = Point::new(1.0, 3.0);

        let cw = vec![
            Triangle::from_points(a.clone(), c.clone(), b.clone()),
            Triangle::from_points(c.clone(), b.clone(), a.clone()),
            Triangle::from_points(b, a, c),
        ];
        for triangle in cw {
            assert!(triangle.area() < 0.0);
        }
    }

    #[test]
    fn test_area_counter_clockwise() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(4.0, 3.0);
        let c = Point::new(1.0, 3.0);

        let ccw = vec![
            Triangle::from_points(a.clone(), b.clone(), c.clone()),
            Triangle::from_points(b.clone(), c.clone(), a.clone()),
            Triangle::from_points(c, a, b),
        ];
        for triangle in ccw {
            assert!(triangle.area() > 0.0);
        }
    }

    #[test]
    fn test_area_collinear() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(4.0, 3.0);
        let c = Point::new(1.0, 3.0);

        // This is choice with replacement over a 3-tuple, so there are
        // 3 * 3 * 3 = 27 total options and this generates all of them.
        let all_combos = std::iter::repeat(vec![&a, &b, &c].into_iter())
            .take(3)
            .multi_cartesian_product();

        for points in all_combos {
            let p0 = points[0];
            let p1 = points[1];
            let p2 = points[2];
            let triangle = Triangle::from_points(p0.clone(), p1.clone(), p2.clone());

            if p0 == p1 || p0 == p2 || p1 == p2 {
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
