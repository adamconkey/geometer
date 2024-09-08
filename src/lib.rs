
use itertools::Itertools;

use crate::triangle::Triangle;
use crate::vertex::Vertex;


pub mod triangle;
pub mod vertex;


#[macro_use]
extern crate lazy_static;


// TODO move polygon to its own file, add some tests for it

pub struct Polygon {
    pub vertices: Vec<Vertex>,
}


impl Polygon {
    // TODO want vec-based new func
    
    pub fn new() -> Polygon {
        Polygon { vertices: Vec::new() }
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        self.vertices.push(v);
    }
    
    pub fn double_area(&self) -> i32 {
        // TODO for now we'll just panic if we access something
        // that doesn't exist, which shouldn't happen if it's
        // a well-defined polygon. So maybe we do some validation
        // to ensure it's valid before even trying computations
        // on it?

        // The first tuple will include the anchor, but that area
        // ends up being zero since p2 will trivially be collinear
        // with anchor-p1 and thus doesn't affect the compuation
        let anchor = &self.vertices[0];
        let mut area = 0;
        for (p1, p2) in self.vertices.iter().tuple_windows() {
            area += Triangle::new(anchor, p1, p2).double_area();
        }
        area
    }
}
