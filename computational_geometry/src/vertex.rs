use std::fmt;

use crate::{
    line_segment::LineSegment,
    point::Point,
};


#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VertexId(u32);

impl From<u32> for VertexId {
    fn from(raw: u32) -> Self {
        Self(raw)
    }
}

impl From<usize> for VertexId {
    fn from(raw: usize) -> Self {
        Self(raw.try_into().unwrap())
    }
}

impl fmt::Display for VertexId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Clone, Debug, PartialEq)]
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

    pub fn rotate_about_origin(&mut self, radians: f64) {
        self.coords.rotate_about_origin(radians);
    }
}