use ordered_float::OrderedFloat as OF;
use std::{cmp::Reverse, collections::HashSet};

use crate::vertex::{Vertex, VertexId};

pub trait Geometry {
    fn vertices(&self) -> Vec<&Vertex>;
    // TODO could be better to make this vec of LineSegment?
    fn edges(&self) -> HashSet<(VertexId, VertexId)>;

    fn get_vertex(&self, id: &VertexId) -> Option<&Vertex>;
    fn get_prev_vertex(&self, id: &VertexId) -> Option<&Vertex>;
    fn get_next_vertex(&self, id: &VertexId) -> Option<&Vertex>;

    fn prev_vertex_id(&self, id: &VertexId) -> Option<VertexId> {
        self.get_prev_vertex(id).map(|v| v.id)
    }

    fn next_vertex_id(&self, id: &VertexId) -> Option<VertexId> {
        self.get_next_vertex(id).map(|v| v.id)
    }

    fn num_edges(&self) -> usize {
        self.edges().len()
    }

    fn num_vertices(&self) -> usize {
        self.vertices().len()
    }

    fn min_x(&self) -> f64 {
        self.vertices().iter().fold(f64::MAX, |acc, v| acc.min(v.x))
    }

    fn max_x(&self) -> f64 {
        self.vertices().iter().fold(f64::MIN, |acc, v| acc.max(v.x))
    }

    fn min_y(&self) -> f64 {
        self.vertices().iter().fold(f64::MAX, |acc, v| acc.min(v.y))
    }

    fn max_y(&self) -> f64 {
        self.vertices().iter().fold(f64::MIN, |acc, v| acc.max(v.y))
    }

    fn leftmost_lowest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (OF(v.y), OF(v.x)));
        vertices[0]
    }

    fn leftmost_highest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (Reverse(OF(v.y)), OF(v.x)));
        vertices[0]
    }

    fn rightmost_lowest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (OF(v.y), Reverse(OF(v.x))));
        vertices[0]
    }

    fn rightmost_highest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (Reverse(OF(v.y)), Reverse(OF(v.x))));
        vertices[0]
    }

    fn lowest_leftmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (OF(v.x), OF(v.y)));
        vertices[0]
    }

    fn lowest_rightmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (Reverse(OF(v.x)), OF(v.y)));
        vertices[0]
    }

    fn highest_leftmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (OF(v.x), Reverse(OF(v.y))));
        vertices[0]
    }

    fn highest_rightmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (Reverse(OF(v.x)), Reverse(OF(v.y))));
        vertices[0]
    }
}

impl<T: Geometry> Geometry for &T {
    fn vertices(&self) -> Vec<&Vertex> {
        (**self).vertices()
    }

    fn edges(&self) -> HashSet<(VertexId, VertexId)> {
        (**self).edges()
    }

    fn get_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        (**self).get_vertex(id)
    }

    fn get_prev_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        (**self).get_prev_vertex(id)
    }

    fn get_next_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        (**self).get_next_vertex(id)
    }
}
