use serde::Deserialize;

use crate::vertex::VertexId;


#[derive(Debug, Deserialize, PartialEq)]
pub struct ConvexHull {
    pub edges: Vec<(VertexId, VertexId)>,
}

impl ConvexHull {
    pub fn new(edges: Vec<(VertexId, VertexId)>) -> Self {
        ConvexHull { edges }
    }

    pub fn default() -> Self {
        let edges = Vec::new();
        Self::new(edges)
    }
}
