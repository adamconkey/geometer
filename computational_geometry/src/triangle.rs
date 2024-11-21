use std::cell::OnceCell;

use crate::{
    point::Point,
    vertex::Vertex,
};


pub struct Triangle<'a> {
    pub p1: &'a Point,
    pub p2: &'a Point,
    pub p3: &'a Point,
    double_area: OnceCell<i32>,
}

impl<'a> Triangle<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point, p3: &'a Point) -> Triangle<'a> {
        Triangle { p1, p2, p3, double_area: OnceCell::new() }
    }

    pub fn from_vertices(v1: &'a Vertex, v2: &'a Vertex, v3: &'a Vertex) -> Triangle<'a> {
        Triangle::new(&v1.coords, &v2.coords, &v3.coords)
    }

    pub fn double_area(&self) -> i32 {
        *self.double_area.get_or_init(|| {
            let t1 = (self.p2.x - self.p1.x) * (self.p3.y - self.p1.y);
            let t2 = (self.p3.x - self.p1.x) * (self.p2.y - self.p1.y);
            t1 - t2
        })
    }

    pub fn area_sign(&self) -> i32 {
        self.double_area().signum()
    }

    pub fn has_collinear_points(&self) -> bool {
        self.area_sign() == 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use crate::vertex::VertexId;

    #[test]
    fn test_from_vertices() {
        let id1 = VertexId::from(1u32);
        let id2 = VertexId::from(2u32);
        let id3 = VertexId::from(3u32);
        let v1 = Vertex::new(Point::new(0, 0), id1, id3, id2);
        let v2 = Vertex::new(Point::new(3, 0), id2, id1, id3);
        let v3 = Vertex::new(Point::new(0, 4), id3, id2, id1);
        let triangle = Triangle::from_vertices(&v1, &v2, &v3);
        assert_eq!(Point::new(0, 0), *triangle.p1);   
        assert_eq!(Point::new(3, 0), *triangle.p2);   
        assert_eq!(Point::new(0, 4), *triangle.p3);   
    }

    #[test]
    fn test_area_right_triangle() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 0);
        let c = Point::new(0, 4);
        let triangle = Triangle::new(&a, &b, &c);
        let double_area = triangle.double_area();
        assert_eq!(double_area, 12);
    }

    // TODO want some better unit tests for the triangle area

    #[test]
    fn test_area_sign_clockwise() {
        let a = Point::new(0, 0);
        let b = Point::new(4, 3);
        let c = Point::new(1, 3);
        
        let cw = vec![
            Triangle::new(&a, &c, &b),
            Triangle::new(&c, &b, &a),
            Triangle::new(&b, &a, &c),
        ];
        for triangle in cw {
            assert_eq!(triangle.area_sign(), -1);
        }
    }

    #[test]
    fn test_area_sign_counter_clockwise() {
        let a = Point::new(0, 0);
        let b = Point::new(4, 3);
        let c = Point::new(1, 3);

        let ccw = vec![
            Triangle::new(&a, &b, &c),
            Triangle::new(&b, &c, &a),
            Triangle::new(&c, &a, &b),
        ];
        for triangle in ccw {
            assert_eq!(triangle.area_sign(), 1);
        }
    }

    #[test]
    fn test_area_sign_collinear() {
        let a = Point::new(0, 0);
        let b = Point::new(4, 3);
        let c = Point::new(1, 3);

        // This is choice with replacement over a 3-tuple, so there are
        // 3 * 3 * 3 = 27 total options and this generates all of them.
        let all_combos = std::iter::repeat(vec![&a, &b, &c].into_iter())
            .take(3)
            .multi_cartesian_product();
        
        for points in all_combos {
            let p0 = points[0];
            let p1 = points[1];
            let p2 = points[2];
            let triangle = Triangle::new(p0, p1, p2);
            
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