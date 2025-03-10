use std::fmt;

use serde::Deserialize;

use crate::{
    line_segment::LineSegment,
    point::Point,
    vector::Vector,
};


#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl fmt::Debug for VertexId {
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

    pub fn right(&self, ab: &LineSegment) -> bool {
        self.coords.right(ab)
    }

    pub fn right_on(&self, ab: &LineSegment) -> bool {
        self.coords.right_on(ab)
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        self.coords.translate(x, y)
    }

    pub fn rotate_about_origin(&mut self, radians: f64) {
        self.coords.rotate_about_origin(radians);
    }

    pub fn rotate_about_point(&mut self, radians: f64, point: &Point) {
        self.coords.rotate_about_point(radians, point);
    }

    pub fn round_coordinates(&mut self) {
        self.coords.round();
    }

    pub fn distance_to(&self, v: &Vertex) -> f64 {
        let vec = Vector::new(v.coords.x - self.coords.x, v.coords.y - self.coords.y);
        vec.magnitude()
    }
}
