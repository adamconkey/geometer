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


impl Vertex {
    
    pub fn new(coords: Point, id: VertexId, prev: VertexId, next: VertexId) -> Vertex {
       Vertex { coords, id, prev, next }
    }

    pub fn between(&self, a: &Vertex, b: &Vertex) -> bool {
        self.coords.between(&a.coords, &b.coords)
    }

    pub fn left(&self, ab: &LineSegment) -> bool {
        self.coords.left(ab)
    }

    pub fn left_on(&self, ab: &LineSegment) -> bool {
        self.coords.left_on(ab)
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
}
