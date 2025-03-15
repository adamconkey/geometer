use ordered_float::OrderedFloat as OF;
use std::{cmp::Reverse, collections::HashSet};

use crate::vertex::{Vertex, VertexId};

pub trait Geometry {
    fn vertices(&self) -> Vec<&Vertex>;
    // TODO could be better to make this vec of LineSegment?
    fn edges(&self) -> HashSet<(VertexId, VertexId)>;

    fn num_edges(&self) -> usize {
        self.edges().len()
    }

    fn num_vertices(&self) -> usize {
        self.vertices().len()
    }

    fn leftmost_lowest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (OF(v.coords.y), OF(v.coords.x)));
        vertices[0]
    }

    fn leftmost_highest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (Reverse(OF(v.coords.y)), OF(v.coords.x)));
        vertices[0]
    }

    fn rightmost_lowest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (OF(v.coords.y), Reverse(OF(v.coords.x))));
        vertices[0]
    }

    fn rightmost_highest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (Reverse(OF(v.coords.y)), Reverse(OF(v.coords.x))));
        vertices[0]
    }

    fn lowest_leftmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (OF(v.coords.x), OF(v.coords.y)));
        vertices[0]
    }

    fn lowest_rightmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (Reverse(OF(v.coords.x)), OF(v.coords.y)));
        vertices[0]
    }

    fn highest_leftmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (OF(v.coords.x), Reverse(OF(v.coords.y))));
        vertices[0]
    }

    fn highest_rightmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| (Reverse(OF(v.coords.x)), Reverse(OF(v.coords.y))));
        vertices[0]
    }
}
