use std::collections::HashMap;
use std::str::FromStr;

use crate::vertex::Vertex;


#[derive(Debug, PartialEq)]
pub struct VertexMap {
    vertices: HashMap<String, Vertex>,
}


#[derive(Debug, PartialEq)]
pub struct ParseVertexMapError;


impl VertexMap {
    pub fn new(vertices: Vec<Vertex>) -> VertexMap {
        let mut vertex_map = HashMap::new();
        for vertex in vertices {
            vertex_map.insert(vertex.id.clone(), vertex);
        }
        VertexMap { vertices: vertex_map }
    }

    pub fn get_vertices(&self, names: Vec<String>) -> Vec<&Vertex> {
        // TODO do better than unwrap here
        names.iter()
            .map(|n| self.vertices.get(n).unwrap())
            .collect()
    }
}


impl FromStr for VertexMap {
    type Err = ParseVertexMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO do better than unwrap here
        let vertices = s.split("\n")
            .map(|vertex_s| Vertex::from_str(vertex_s).unwrap())
            .collect();
        
        Ok(VertexMap::new(vertices))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let a = Vertex::new_with_id(1, 2, String::from("a"));
        let b = Vertex::new_with_id(3, 4, String::from("b"));
        let expected_vertices = vec![a, b];
        let expected_vmap = VertexMap::new(expected_vertices);
        let input = "1 2 a\n3 4 b";
        let vmap = VertexMap::from_str(input).unwrap();
        assert_eq!(vmap, expected_vmap);
    }
}
