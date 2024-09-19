
use itertools::Itertools;
use std::collections::HashMap;

use crate::line_segment::LineSegment;
use crate::triangle::Triangle;
use crate::vertex::Vertex;


pub struct Polygon<'a> {
    anchor: &'a Vertex,
    // TODO might need to generalize beyond tuples for e.g.
    // triangulations, but currently it makes sense the way
    // polygons are defined as a circular chain, every
    // vertex has exactly two neighbors. This is effectively
    // hacking around a dll implementation for rust
    neighbors: HashMap<&'a Vertex, (&'a Vertex, &'a Vertex)>,
}


impl<'a> Polygon<'a> {
    pub fn new(vertices: Vec<&'a Vertex>) -> Polygon<'a> {
        let mut neighbors = HashMap::new();

        let first = &vertices[0];
        let last = vertices
            .last()
            .expect("Polygon should have at least 3 vertices");

        // TODO I suspect I'm doing something silly here, having to deref
        // everything. I think it has to do with me storing a vec of refs
        // and then doing iter as opposed to into_iter()
        
        for (v0, v1, v2) in vertices.iter().tuple_windows::<(_,_,_)>() {
            neighbors.insert(*v1, (*v0, *v2));

            if v0 == first {
                neighbors.insert(*v0, (*last, *v1));
            }
            if v2 == last {
                neighbors.insert(*v2, (*v1, *first));
            }
        }

        Polygon { anchor: &vertices[0], neighbors: neighbors }
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
        let mut area = 0;
        for e in self.edges() {
            area += Triangle::new(self.anchor, e.v1, e.v2).double_area();
        }
        area
    }

    pub fn edges(&self) -> Vec<LineSegment> {
        // TODO would be cool to cache this
        let mut edges = Vec::new();
        let mut current = self.anchor;

        // Do forward pass through hashmap to get all ordered edges
        loop {
            let (_prev, next) = self.neighbors
                .get(current)
                .expect("Every vertex should have neighbors stored");
            edges.push(LineSegment::new(current, next));

            current = next;
            
            if current == self.anchor {
                break;
            }
        }

        edges
    }

    pub fn neighbors(&self, v: &Vertex) -> (&Vertex, &Vertex) {
        *self.neighbors
            .get(v)
            .expect("Every vertex should have neighbors stored")
        
    }
        
    //     // TODO this is a downside of my simple starting approach
    //     // to storing vertices in a vec instead of doing dll which
    //     // I think is going to be just too onerous in Rust. Have
    //     // to do a O(n) search for the vertex. Could investigate
    //     // more efficient ways to handle this as it fundamentally
    //     // is a simple adjacency structure for a polygon where
    //     // each vertex is adjacent to exactly 2 vertices. I'm
    //     // not encoding arbitrary graphs so can likely do
    //     // something better here.

    //     // TODO this won't work if vertex not in neighbors, would
    //     // want to add error checking but really this is a terrible
    //     // way to get neighbors, want to rethink this.
        
    //     let first = &self.vertices[0];
    //     let last = self.vertices
    //         .last()
    //         .expect("Polygon should have at least 3 vertices");

    //     // Arbitrarily initializing, these will be updated below
    //     let mut prev: &Vertex = first;
    //     let mut next: &Vertex = first;

    //     // Edge case that won't be picked up by linear search if
    //     // the vertex we're looking for is the first in our vec
    //     if v == first {
    //         prev = last;
    //         next = &self.vertices[1];
    //         return (prev, next);
    //     }
        
    //     for (v0, v1, v2) in self.vertices.iter().tuple_windows::<(_,_,_)>() {
    //         if v == v1 {
    //             prev = v0;
    //             next = v2;
    //             return (prev, next);
    //         } else if v2 == last {
    //             // Edge case if the vertex we're looking for happens to be
    //             // the last in the vec, need to wrap around to the front
    //             prev = v1;
    //             next = first;
    //             return (prev, next);
    //         }
    //     }

    //     (prev, next)
    // }

    // TODO need to add tests for all of these, in_cone, diagonal, diag i/e
    
    // pub fn in_cone(&self, ab: &LineSegment) -> bool {
    //     let a = ab.v1;
    //     let ba = &ab.reverse();
    //     let (a0, a1) = self.neighbors(a);

    //     // TODO I feel like 'reflexive' is perhaps a more meaningful
    //     // property than left_on, maybe think about adding that to
    //     // polylgon and reversing the checks here?
        
    //     if a0.left_on(&LineSegment::new(a, a1)) {
    //         return a0.left(ab) && a1.left(ba);
    //     }
        
    //     // Otherwise a is reflexive
    //     !(a1.left_on(ab) && a0.left_on(ba))
    // }
    
    // pub fn diagonal(&self, ab: &LineSegment) -> bool {
    //     let ba = &ab.reverse();
    //     self.in_cone(ab) && self.in_cone(ba) && self.diagonal_internal_external(ab)
    // }

    // fn diagonal_internal_external(&self, ab: &LineSegment) -> bool {
    //     for e in self.edges() {
    //         if !e.connected_to(ab) && e.intersects(ab) {
    //             return false;
    //         }
    //     } 
    //     true
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_right_triangle() {
        let a = Vertex::new(0, 0);
        let b = Vertex::new(3, 0);
        let c = Vertex::new(0, 4);
        let vertices = vec![&a, &b, &c];
        let polygon = Polygon::new(vertices);
        let double_area = polygon.double_area();
        assert_eq!(double_area, 12);
    }

    #[test]
    fn test_neighbors_square() {
        let a = Vertex::new(0, 0);
        let b = Vertex::new(4, 0);
        let c = Vertex::new(4, 4);
        let d = Vertex::new(0, 4);
        let vertices = vec![&a, &b, &c, &d];
        let polygon = Polygon::new(vertices);

        assert_eq!(polygon.neighbors(&a), (&d, &b));
        assert_eq!(polygon.neighbors(&b), (&a, &c));
        assert_eq!(polygon.neighbors(&c), (&b, &d));
        assert_eq!(polygon.neighbors(&d), (&c, &a));
    }

    // #[test]
    // fn test_neighbors_p1() {
    //     // TODO will be using polygons across tests, would be great to
    //     // have better way to configure this, not sure what fixtures
    //     // Rust offers
    //     let a = Vertex::new( 0, 0);
    //     let b = Vertex::new( 3, 4);
    //     let c = Vertex::new( 6, 2);
    //     let d = Vertex::new( 7, 6);
    //     let e = Vertex::new( 3, 9);
    //     let f = Vertex::new(-2, 7);

    //     // TODO need to rethink this whole thing, I thought it would make
    //     // sense for a polygon to own the vertices, it still might, but
    //     // it does make it more of a pain to test things.
    //     let mut polygon = Polygon::new();
    //     polygon.add_vertex(a.clone());
    //     polygon.add_vertex(b.clone());
    //     polygon.add_vertex(c.clone());
    //     polygon.add_vertex(d.clone());
    //     polygon.add_vertex(e.clone());
    //     polygon.add_vertex(f.clone());

    //     assert_eq!(polygon.neighbors(&a), (&f, &b));
    //     assert_eq!(polygon.neighbors(&b), (&a, &c));
    //     assert_eq!(polygon.neighbors(&c), (&b, &d));
    //     assert_eq!(polygon.neighbors(&d), (&c, &e));
    //     assert_eq!(polygon.neighbors(&e), (&d, &f));
    //     assert_eq!(polygon.neighbors(&f), (&e, &a));
    // }
    
    // #[test]
    // fn test_diagonal() {
    //     let a = Vertex::new( 0, 0);
    //     let b = Vertex::new( 3, 4);
    //     let c = Vertex::new( 6, 2);
    //     let d = Vertex::new( 7, 6);
    //     let e = Vertex::new( 3, 9);
    //     let f = Vertex::new(-2, 7);

    //     // TODO need to rethink this whole thing, I thought it would make
    //     // sense for a polygon to own the vertices, it still might, but
    //     // it does make it more of a pain to test things.
    //     let mut polygon = Polygon::new();
    //     polygon.add_vertex(a.clone());
    //     polygon.add_vertex(b.clone());
    //     polygon.add_vertex(c.clone());
    //     polygon.add_vertex(d.clone());
    //     polygon.add_vertex(e.clone());
    //     polygon.add_vertex(f.clone());

    //     // TODO obviously this is a pain, should think more about line
    //     // segment and whether it's going to be a pain to have as a
    //     // primitive, the alternative could be to just use ordered
    //     // vertices (or assume order on vertices when input to funcs)
    //     let ac = LineSegment::new(&a, &c);
    //     let ad = LineSegment::new(&a, &d);
    //     let ae = LineSegment::new(&a, &e);
    //     let bd = LineSegment::new(&b, &d);
    //     let be = LineSegment::new(&b, &e);
    //     let bf = LineSegment::new(&b, &f);
    //     let ca = LineSegment::new(&c, &a);
    //     let ce = LineSegment::new(&c, &e);
    //     let cf = LineSegment::new(&c, &f);
    //     let da = LineSegment::new(&d, &a);
    //     let db = LineSegment::new(&d, &b);
    //     let df = LineSegment::new(&d, &f);
    //     let ea = LineSegment::new(&e, &a);
    //     let eb = LineSegment::new(&e, &b);
    //     let ec = LineSegment::new(&e, &c);
    //     let fb = LineSegment::new(&f, &b);
    //     let fc = LineSegment::new(&f, &c);
    //     let fd = LineSegment::new(&f, &d);

    //     let internal = vec![&ae, &bd, &be, &bf, &ce, &db, &df, &ea, &eb, &ec, &fb, &fd];
    //     let external = vec![&ac, &ca];
    //     let not_diagonal = vec![&ad, &cf, &da, &fc];
        
    //     for ls in internal {
    //         assert!(polygon.in_cone(ls));
    //         assert!(polygon.diagonal_internal_external(ls));
    //         assert!(polygon.diagonal(ls));
    //     }

    //     for ls in external {
    //         // TODO might want to think of another example and think carefully
    //         // about the in_cone, I think there'd be examples where at least
    //         // one of the directions fails
    //         assert!(!polygon.in_cone(ls));
    //         assert!( polygon.diagonal_internal_external(ls));
    //         assert!(!polygon.diagonal(ls));
    //     }

    //     for ls in not_diagonal {
    //         assert!(!polygon.diagonal_internal_external(ls));
    //         assert!(!polygon.diagonal(ls));
    //     }
    // }
}
