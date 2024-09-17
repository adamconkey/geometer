
use itertools::Itertools;


use crate::line_segment::LineSegment;
use crate::triangle::Triangle;
use crate::vertex::Vertex;


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

        // The first edge will include the anchor, but that area
        // ends up being zero since v2 will trivially be collinear
        // with anchor-v1 and thus doesn't affect the compuation
        let anchor = &self.vertices[0];
        let mut area = 0;
        for e in self.edges() {
            area += Triangle::new(anchor, e.v1, e.v2).double_area();
        }
        area
    }

    pub fn edges(&self) -> Vec<LineSegment> {
        // TODO not sure if this is a bad idea or not, doing this
        // iter to get the tuples and then map/collect when in
        // most cases we'll probably just be iterating over the
        // edges. But it is fairly legible so let's give it a go
        self.vertices.iter().tuple_windows()
            .map(|(p1, p2)| LineSegment::new(p1, p2))
            .collect()
    }

    // TODO implement cone functions (used for diagonal check)
    // TODO implement public diagonal function after cone functions

    fn _diagonal_internal_external(&self, _ab: &LineSegment) -> bool {
        for _e in self.edges() {
            // TODO continue algorithm, should check for intersection
            // of edge against ab, skipping any that are incident to
            // ab. Might be good to add an incident func on LineSegment
            // for a vertex. This block will short-circuit return
            // false if a non-incident edge (of a or b) intersects
            // ab. 
        }
        
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_right_triangle() {
        let a = Vertex::new(0, 0);
        let b = Vertex::new(3, 0);
        let c = Vertex::new(0, 4);
        let mut polygon = Polygon::new();
        polygon.add_vertex(a);
        polygon.add_vertex(b);
        polygon.add_vertex(c);
        let double_area = polygon.double_area();
        assert_eq!(double_area, 12);
    }
}
