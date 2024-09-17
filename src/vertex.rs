
use unique_id::Generator;
use unique_id::sequence::SequenceGenerator;

use crate::line_segment::LineSegment;
use crate::triangle::Triangle;


lazy_static!(
    static ref ID_GENERATOR: SequenceGenerator = SequenceGenerator::default();
);


// TODO make this type-generic?
// TODO handle dimensionality of coordinates
#[derive(Debug, PartialEq)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub index: i64,
}


impl Vertex {
    pub fn new(x: i32, y: i32) -> Vertex {
        Vertex { x: x, y: y, index: ID_GENERATOR.next_id() }
    }

    pub fn between(&self, a: &Vertex, b: &Vertex) -> bool {
        if !Triangle::new(a, b, self).has_collinear_points() {
            return false;
        }

        let (e1, e2, check) = match LineSegment::new(&a, &b).is_vertical() {
            true  => (a.y, b.y, self.y),
            false => (a.x, b.x, self.x),
        };
        
        (e1..e2).contains(&check) || (e2..e1).contains(&check)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let v0 = Vertex::new(1, 2);
        let v1 = Vertex::new(3, 4);
        assert_ne!(v0.index, v1.index);
    }

    #[test]
    fn test_between() {
        let v0 = Vertex::new(0, 0);
        let v1 = Vertex::new(1, 1);
        let v2 = Vertex::new(2, 2);

        assert!( v1.between(&v0, &v2));
        assert!( v1.between(&v2, &v1));
        assert!(!v0.between(&v1, &v2));
        assert!(!v0.between(&v2, &v1));
        assert!(!v2.between(&v0, &v1));
        assert!(!v2.between(&v1, &v0));
    }

    #[test]
    fn test_between_vertical() {
        let v0 = Vertex::new(0, 0);
        let v1 = Vertex::new(0, 1);
        let v2 = Vertex::new(0, 2);

        assert!( v1.between(&v0, &v2));
        assert!( v1.between(&v2, &v1));
        assert!(!v0.between(&v1, &v2));
        assert!(!v0.between(&v2, &v1));
        assert!(!v2.between(&v0, &v1));
        assert!(!v2.between(&v1, &v0));
    }
}
