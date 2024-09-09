
use crate::vertex::Vertex;


pub struct Triangle<'a> {
    pub a: &'a Vertex,
    pub b: &'a Vertex,
    pub c: &'a Vertex,
}


impl<'a> Triangle<'a> {
    pub fn new(a: &'a Vertex, b: &'a Vertex, c: &'a Vertex) -> Triangle<'a> {
        Triangle {
            a: a,
            b: b,
            c: c,
        }
    }

    pub fn double_area(&self) -> i32 {
        let t1 = (self.b.x - self.a.x) * (self.c.y - self.a.y);
        let t2 = (self.c.x - self.a.x) * (self.b.y - self.a.y);
        t1 - t2
    }

    pub fn area_sign(&self) -> i32 {
        self.double_area().signum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_right_triangle() {
        let a = Vertex::new(0, 0);
        let b = Vertex::new(3, 0);
        let c = Vertex::new(0, 4);
        let triangle = Triangle::new(&a, &b, &c);
        let double_area = triangle.double_area();
        assert_eq!(double_area, 12);
    }

    // TODO want some better unit tests for the triangle area

    #[test]
    fn test_area_sign_clockwise() {
        let a = Vertex::new(0, 0);
        let b = Vertex::new(4, 3);
        let c = Vertex::new(1, 3);
        
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
        let a = Vertex::new(0, 0);
        let b = Vertex::new(4, 3);
        let c = Vertex::new(1, 3);

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
        let a = Vertex::new(0, 0);
        let b = Vertex::new(4, 3);
        let c = Vertex::new(1, 3);
        
        let collinear = vec![
            Triangle::new(&a, &a, &a),
            Triangle::new(&a, &a, &b),
            Triangle::new(&a, &a, &c),
            Triangle::new(&a, &b, &a),
            Triangle::new(&a, &b, &b),
            Triangle::new(&a, &c, &a),
            Triangle::new(&a, &c, &c),
            Triangle::new(&b, &b, &a),
            Triangle::new(&b, &b, &b),
            Triangle::new(&b, &b, &c),
            Triangle::new(&b, &a, &a),
            Triangle::new(&b, &a, &b),
            Triangle::new(&b, &c, &b),
            Triangle::new(&b, &c, &c),
            Triangle::new(&c, &c, &a),
            Triangle::new(&c, &c, &b),
            Triangle::new(&c, &c, &c),
            Triangle::new(&c, &a, &a),
            Triangle::new(&c, &a, &c),
            Triangle::new(&c, &b, &b),
            Triangle::new(&c, &b, &c),
        ];
        for triangle in collinear {
            assert_eq!(triangle.area_sign(), 0);
        }
    }
}
