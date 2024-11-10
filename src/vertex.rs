use serde::{Deserialize, Serialize};
use unique_id::Generator;
use unique_id::sequence::SequenceGenerator;

use crate::{
    line_segment::LineSegment,
    point::Point,
    triangle::Triangle,
};


lazy_static!(
    static ref ID_GENERATOR: SequenceGenerator = SequenceGenerator::default();
);


#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VertexId(i64);

impl VertexId {
    pub fn new(id: Option<i64>) -> Self{
        if let Some(id_val) = id {
            VertexId(id_val)
        }
        else {
            VertexId(ID_GENERATOR.next_id())
        }
    }
}

impl From<i64> for VertexId {
    fn from(raw: i64) -> Self {
        Self(raw)
    }
}


// TODO make this type-generic?
// TODO handle dimensionality of coordinates
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vertex {
    pub coords: Point,
    pub id: VertexId,
    pub prev: VertexId,
    pub next: VertexId,
}


#[derive(Debug, PartialEq)]
pub struct ParseVertexError;


impl Vertex {
    
    pub fn new(coords: Point, id: VertexId, prev: VertexId, next: VertexId) -> Vertex {
       Vertex { coords, id, prev, next }
    }

    pub fn between(&self, a: &Vertex, b: &Vertex) -> bool {
        if !Triangle::new(a, b, self).has_collinear_points() {
            return false;
        }

        let (e1, e2, check) = match LineSegment::new(&a, &b).is_vertical() {
            true  => (a.coords.y, b.coords.y, self.coords.y),
            false => (a.coords.x, b.coords.x, self.coords.x),
        };
        
        (e1..e2).contains(&check) || (e2..e1).contains(&check)
    }

    pub fn left(&self, ab: &LineSegment) -> bool {
        Triangle::new(ab.v1, ab.v2, self).area_sign() > 0
    }

    pub fn left_on(&self, ab: &LineSegment) -> bool {
        Triangle::new(ab.v1, ab.v2, self).area_sign() >= 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ids_distinct() {
        let id1 = VertexId::new(None);
        let id2 = VertexId::new(None);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_id_with_input_value() {
        let val: i64 = 1234;
        let new_id = VertexId::new(Some(val));
        let from_id = VertexId::from(val);
        assert_eq!(from_id, new_id);
    }

    // // TODO might be nice to add custom macro for between asserts,
    // // not sure how difficult it is to write macros at this stage
    // #[test]
    // fn test_between() {
    //     let v0 = Vertex::new(0, 0);
    //     let v1 = Vertex::new(1, 1);
    //     let v2 = Vertex::new(2, 2);

    //     assert!( v1.between(&v0, &v2));
    //     assert!( v1.between(&v2, &v1));
    //     assert!(!v0.between(&v1, &v2));
    //     assert!(!v0.between(&v2, &v1));
    //     assert!(!v2.between(&v0, &v1));
    //     assert!(!v2.between(&v1, &v0));
    // }

    // #[test]
    // fn test_between_vertical() {
    //     let v0 = Vertex::new(0, 0);
    //     let v1 = Vertex::new(0, 1);
    //     let v2 = Vertex::new(0, 2);

    //     assert!( v1.between(&v0, &v2));
    //     assert!( v1.between(&v2, &v1));
    //     assert!(!v0.between(&v1, &v2));
    //     assert!(!v0.between(&v2, &v1));
    //     assert!(!v2.between(&v0, &v1));
    //     assert!(!v2.between(&v1, &v0));
    // }
}
