use std::{fmt, slice::Iter};

use crate::{point::Point, polygon::Polygon, vertex::VertexId};


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
    triangles: Vec<TriangleVertexIds>,
    polygon: &'a Polygon,
}

impl<'a> Triangulation<'a> {
    pub fn new(polygon: &'a Polygon) -> Self {
        Self { triangles: Vec::new(), polygon }
    }    
    
    pub fn push(&mut self, value: TriangleVertexIds) {
        self.triangles.push(value)
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
                    self.polygon.get_point(&ids.0).unwrap(),
                    self.polygon.get_point(&ids.1).unwrap(),
                    self.polygon.get_point(&ids.2).unwrap(),
                )
            ).collect()        
    }
}