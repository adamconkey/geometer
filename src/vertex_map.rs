use indexmap::IndexMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::line_segment::LineSegment;
use crate::vertex::Vertex;


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct VertexMap {
    vertices: IndexMap<String, Vertex>,
}

#[derive(Debug, PartialEq)]
pub struct ParseVertexMapError;


// impl VertexMap {
//     pub fn new(vertices: Vec<Vertex>) -> VertexMap {
//         let mut vertex_map = IndexMap::new();
//         for vertex in vertices {
//             vertex_map.insert(vertex.id.clone(), vertex);
//         }
//         VertexMap { vertices: vertex_map }
//     }

//     pub fn from_json<P: AsRef<Path>>(path: P) -> VertexMap {
//         let polygon_str: String = fs::read_to_string(path)
//             .expect("file should exist and be parseable");
//         // TODO don't unwrap
//         serde_json::from_str(&polygon_str).unwrap()
//     }

//     pub fn get(&self, id: &str) -> Option<&Vertex> {
//         self.vertices.get(id)
//     }

//     pub fn all_vertices(&self) -> Vec<&Vertex> {
//         self.vertices.values().collect()
//     }
    
//     pub fn get_vertices(&self, names: Vec<String>) -> Vec<&Vertex> {
//         // TODO do better than unwrap here
//         names.iter()
//             .map(|id| self.get(id).unwrap())
//             .collect()
//     }

//     pub fn get_line_segment(&self, id_1: &str, id_2: &str) -> LineSegment {
//         // TODO this should return Option<LineSegment> and handle 
//         // the cases below instead of unwrap here
//         let v1 = self.get(id_1).unwrap();
//         let v2 = self.get(id_2).unwrap();
//         LineSegment::new(v1, v2)
//     }
// }


// impl FromStr for VertexMap {
//     type Err = ParseVertexMapError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // TODO do better than unwrap here
//         let vertices = s.split("\n")
//             .map(|vertex_s| Vertex::from_str(vertex_s).unwrap())
//             .collect();
        
//         Ok(VertexMap::new(vertices))
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from_str() {
//         let a = Vertex::new_with_id(1, 2, String::from("a"));
//         let b = Vertex::new_with_id(3, 4, String::from("b"));
//         let expected_vertices = vec![a, b];
//         let expected_vmap = VertexMap::new(expected_vertices);
//         let input = "1 2 a\n3 4 b";
//         let vmap = VertexMap::from_str(input).unwrap();
//         assert_eq!(vmap, expected_vmap);
//     }

//     #[test]
//     fn test_serialization() {
//         let a = Vertex::new_with_id(1, 2, String::from("a"));
//         let b = Vertex::new_with_id(3, 4, String::from("b"));
//         let vertices = vec![a, b];
//         let vmap = VertexMap::new(vertices);

//         let expected_serialized = r#"{"vertices":{"a":{"x":1,"y":2,"id":"a"},"b":{"x":3,"y":4,"id":"b"}}}"#;

//         let serialized = serde_json::to_string(&vmap).unwrap();
//         let deserialized: VertexMap = serde_json::from_str(&serialized).unwrap();
        
//         assert_eq!(serialized, expected_serialized);
//         assert_eq!(deserialized, vmap);
//     }
// }
