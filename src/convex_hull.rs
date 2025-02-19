use serde::Deserialize;

use crate::vertex::VertexId;


#[derive(Debug, Deserialize, PartialEq)]
pub struct Edge(pub VertexId, pub VertexId);


#[derive(Debug, Deserialize, PartialEq)]
pub struct ConvexHull {
    pub edges: Vec<Edge>,
}

impl ConvexHull {
    pub fn new(edges: Vec<Edge>) -> Self {
        ConvexHull { edges }
    }

    pub fn default() -> Self {
        let edges = Vec::new();
        Self::new(edges)
    }
}
