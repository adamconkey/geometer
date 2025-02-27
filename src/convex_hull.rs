use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{polygon::Polygon, vertex::VertexId};


#[derive(Debug)]
pub struct ConvexHull<'a> {
    // Using Reverse makes this a min-heap, so 
    // that we get CCW order going min to max
    vertices: BinaryHeap<Reverse<VertexId>>,
    polygon: &'a Polygon,
}

impl<'a> ConvexHull<'a> {
    pub fn new(polygon: &'a Polygon) -> Self {
        let vertices = BinaryHeap::new();
        ConvexHull { vertices, polygon }
    }

    pub fn push(&mut self, vertex_id: VertexId) {
        self.vertices.push(Reverse(vertex_id));
    }
}
