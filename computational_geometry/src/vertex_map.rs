use std::collections::{hash_map, HashMap};

use crate::point::Point;
use crate::vertex::{Vertex, VertexId};


#[derive(Clone, Debug, PartialEq)]
pub struct VertexMap {
    map: HashMap<VertexId, Vertex>,
}

impl VertexMap {
    pub fn new(points: Vec<Point>) -> Self {
        let mut map = HashMap::new();

        // TODO currently the IDs are simply generated starting
        // at 0 and incrementing. If you want to keep this route,
        // will need to track index on self so that new vertices
        // could be added. Tried using unique_id::SequenceGenerator
        // but it was global which was harder to test with
        let num_points = points.len();
        let vertex_ids = (0..num_points)
            .map(VertexId::from)
            .collect::<Vec<_>>();

        for (i, point) in points.into_iter().enumerate() {
            let prev_id = vertex_ids[(i + num_points - 1) % num_points];
            let curr_id = vertex_ids[i];
            let next_id = vertex_ids[(i + num_points + 1) % num_points];
            let v = Vertex::new(point, curr_id, prev_id, next_id);
            map.insert(curr_id, v);
        }

        VertexMap { map }
    }

    pub fn get(&self, k: &VertexId) -> &Vertex {
        self.map.get(k).unwrap()
    }

    pub fn get_mut(&mut self, k: &VertexId) -> &mut Vertex{
        self.map.get_mut(k).unwrap()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn remove(&mut self, k: &VertexId) -> Vertex {
        let v = self.map.remove(k).unwrap();
        self.update_next(&v.prev, &v.next);
        self.update_prev(&v.next, &v.prev);
        v
    }

    pub fn values(&self) -> hash_map::Values<'_, VertexId, Vertex> {
        self.map.values()
    }

    pub fn sorted_vertices(&self) -> Vec<&Vertex> {
        let mut vertices = self.values()
            .collect::<Vec<&Vertex>>();
        vertices.sort_by(|a, b| a.id.cmp(&b.id));
        vertices
    }

    pub fn sorted_points(&self) -> Vec<Point> {
        self.sorted_vertices()
            .iter()
            .map(|v| v.coords.clone())
            .collect::<Vec<Point>>()
    }

    pub fn anchor(&self) -> &Vertex {
        // TODO I'm not yet convinced this is something I want, ultimately
        // need something to initiate algorithms in the vertex chain.
        // Could consider only exposing keys and then having polygon gen
        // an anchor
        self.values().collect::<Vec<_>>()[0]
    }

    pub fn update_next(&mut self, k: &VertexId, next: &VertexId) {
        self.get_mut(k).next = *next;
    }

    pub fn update_prev(&mut self, k: &VertexId, prev: &VertexId) {
        self.get_mut(k).prev = *prev;
    }
}