use std::collections::HashMap;

use crate::vertex::Vertex;


pub struct VertexMap {
    vertices: HashMap<String, Vertex>,
}


impl VertexMap {
    pub fn new(vertices: Vec<Vertex>) -> VertexMap {
        let mut vertex_map = HashMap::new();
        for vertex in vertices {
            vertex_map.insert(vertex.id.clone(), vertex);
        }
        VertexMap { vertices: vertex_map }
    }

    pub fn get_vertices(&self, names: Vec<String>) -> Vec<&Vertex> {
        names.iter()
            .map(|n| self.vertices.get(n).expect("Should have it"))
            .collect()
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from_str() {
//         let _input = "1 2 a\n3 4 b";
//     }
// }
