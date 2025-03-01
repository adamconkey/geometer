use std::collections::HashSet;

use itertools::Itertools;

use crate::vertex::VertexId;


#[derive(Debug)]
pub struct ConvexHull {
    vertices: HashSet<VertexId>,
}

impl ConvexHull {
    pub fn new(vertices: HashSet<VertexId>) -> Self {
        ConvexHull { vertices }
    }

    pub fn add_vertex(&mut self, vertex_id: VertexId) {
        self.vertices.insert(vertex_id);
    }

    pub fn add_vertices(&mut self, vertices: impl IntoIterator<Item = VertexId>) {
        for v in vertices {
            self.add_vertex(v);
        }
    }

    pub fn get_vertices(&self) -> Vec<VertexId> {
        // TODO this is a little silly to sort on every retrieval 
        // especially if nothing changes. I originally had a 
        // min-heap but found you still had to clone/sort to get 
        // the whole vec (not just peek/pop from top), so just 
        // doing vec seemed simpler, then I added lazy sorting
        // but realized if going that route, should just maintain
        // sort order on input. Long story short there are smarter
        // ways to do this but this is simple and fast, should
        // do this smarter at a later time. 
        let mut sorted = self.vertices.iter().cloned().collect_vec();
        sorted.sort();
        sorted
    }
}

impl PartialEq for ConvexHull {
    fn eq(&self, other: &Self) -> bool {
        self.get_vertices() == other.get_vertices()
    }
}

impl Default for ConvexHull {
    fn default() -> Self {
        let vertices = HashSet::new();
        ConvexHull { vertices }
    }
}