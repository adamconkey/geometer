use std::collections::{hash_map, HashMap};

use crate::point::Point;
use crate::vertex::{Vertex, VertexId};


#[derive(Clone, Debug, PartialEq)]
pub struct VertexMap(HashMap<VertexId, Vertex>);


impl VertexMap {
    pub fn new(points: Vec<Point>) -> Self {
        let mut vertex_map = HashMap::new();

        let num_points = points.len();
        let vertex_ids = (0..num_points)
            .map(|_| VertexId::new(None))
            .collect::<Vec<_>>();

        for (i, point) in points.into_iter().enumerate() {
            let prev_id = vertex_ids[(i + num_points - 1) % num_points];
            let curr_id = vertex_ids[i];
            let next_id = vertex_ids[(i + num_points + 1) % num_points];
            let v = Vertex::new(point, curr_id, prev_id, next_id);
            vertex_map.insert(curr_id, v);
        }

        Self(vertex_map)
    }

    pub fn get(&self, k: &VertexId) -> &Vertex {
        // Unwrapping since this is for internal use only
        // and it will be assumed that internally we only 
        // operate on valid IDs in the map
        self.0.get(k).unwrap()
    }

    pub fn get_mut(&mut self, k: &VertexId) -> &mut Vertex{
        self.0.get_mut(k).unwrap()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn remove(&mut self, k: &VertexId) -> Vertex {
        self.0.remove(k).unwrap()
    }

    pub fn values(&self) -> hash_map::Values<'_, VertexId, Vertex> {
        self.0.values()
    }

    pub fn anchor(&self) -> &Vertex {
        // TODO I'm not yet convinced this is something I want, ultimately
        // need something to initiate algorithms in the vertex chain.
        // Could consider only exposing keys and then having polygon gen
        // an anchor
        self.values().collect::<Vec<_>>()[0]
    }

    pub fn update_next(&mut self, k: &VertexId, next: &VertexId) {
        self.get_mut(k).next = next.clone();
    }

    pub fn update_prev(&mut self, k: &VertexId, prev: &VertexId) {
        self.get_mut(k).prev = prev.clone();
    }
}
