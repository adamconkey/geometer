use std::{collections::{hash_set::Iter, HashSet}, fmt};

use crate::{point::Point, vertex::VertexId, vertex_map::VertexMap};


#[derive(Debug, Clone)]
pub struct EarNotFoundError;

impl fmt::Display for EarNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "polygon is likely invalid")
    }
}


#[derive(Eq, Hash, PartialEq)]
pub struct TriangleVertexIds(pub VertexId, pub VertexId, pub VertexId);


pub struct Triangulation<'a> {
    triangles: HashSet<TriangleVertexIds>,
    vmap: &'a VertexMap,
}

impl<'a> Triangulation<'a> {
    pub fn new(vmap: &'a VertexMap) -> Self {
        Self { triangles: HashSet::new(), vmap }
    }    
    
    pub fn insert(&mut self, value: TriangleVertexIds) -> bool {
        self.triangles.insert(value)
    }

    pub fn iter(&self) -> Iter<'_, TriangleVertexIds> { 
        self.triangles.iter()
    }

    pub fn len(&self) -> usize {
        self.triangles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.triangles.is_empty()
    }

    pub fn to_points(&self) -> Vec<(Point, Point, Point)> {
        self.triangles.iter()
            .map(|ids| 
                (
                    self.vmap.get(&ids.0).coords.clone(),
                    self.vmap.get(&ids.1).coords.clone(),
                    self.vmap.get(&ids.2).coords.clone()
                )
            ).collect()        
    }
}