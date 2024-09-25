use itertools::Itertools;
use std::collections::HashMap;

use crate::{
    line_segment::LineSegment,
    triangle::Triangle,
    vertex::Vertex,
    vertex_map::VertexMap,
};


pub struct Polygon<'a> {
    anchor: &'a Vertex,
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
    
    pub fn in_cone(&self, ab: &LineSegment) -> bool {
        let a = ab.v1;
        let ba = &ab.reverse();
        let (a0, a1) = self.neighbors(a);

        // TODO I feel like 'reflexive' is perhaps a more meaningful
        // property than left_on, maybe think about adding that to
        // polylgon and reversing the checks here?
        
        if a0.left_on(&LineSegment::new(a, a1)) {
            return a0.left(ab) && a1.left(ba);
        }
        
        // Otherwise a is reflexive
        !(a1.left_on(ab) && a0.left_on(ba))
    }
    
    pub fn diagonal(&self, ab: &LineSegment) -> bool {
        let ba = &ab.reverse();
        self.in_cone(ab) && self.in_cone(ba) && self.diagonal_internal_external(ab)
    }

    fn diagonal_internal_external(&self, ab: &LineSegment) -> bool {
        for e in self.edges() {
            if !e.connected_to(ab) && e.intersects(ab) {
                return false;
            }
        } 
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_right_triangle() {
        let a = Vertex::new_with_id(0, 0, String::from("a"));
        let b = Vertex::new_with_id(3, 0, String::from("b"));
        let c = Vertex::new_with_id(0, 4, String::from("c"));

        // TODO trying this out, ultimately creating a version that will
        // read from strings and then files. So need to implement from_str
        // for vertex map, then you have an owner for "sets" of vertices
        // and then can pass to arbitrary polygons as refs. Should hopefully
        // sidestep ownership issues in polygons, and then allow for reading
        // polygons more easily from file/strings to make tests lighter
        // weight. Will want to implement string-based getter on the map
        // struct so that you can easily retrieve for asserts based on id
        let vertex_map = VertexMap::new(vec![a, b, c]);
        let vertex_names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let vertices = vertex_map.get_vertices(vertex_names);
        
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

    #[test]
    fn test_neighbors_p1() {
        // TODO will be using polygons across tests, would be great to
        // have better way to configure this, not sure what fixtures
        // Rust offers
        let a = Vertex::new( 0, 0);
        let b = Vertex::new( 3, 4);
        let c = Vertex::new( 6, 2);
        let d = Vertex::new( 7, 6);
        let e = Vertex::new( 3, 9);
        let f = Vertex::new(-2, 7);
        let vertices = vec![&a, &b, &c, &d, &e, &f];
        let polygon = Polygon::new(vertices);

        assert_eq!(polygon.neighbors(&a), (&f, &b));
        assert_eq!(polygon.neighbors(&b), (&a, &c));
        assert_eq!(polygon.neighbors(&c), (&b, &d));
        assert_eq!(polygon.neighbors(&d), (&c, &e));
        assert_eq!(polygon.neighbors(&e), (&d, &f));
        assert_eq!(polygon.neighbors(&f), (&e, &a));
    }
    
    #[test]
    fn test_diagonal() {
        let a = Vertex::new( 0, 0);
        let b = Vertex::new( 3, 4);
        let c = Vertex::new( 6, 2);
        let d = Vertex::new( 7, 6);
        let e = Vertex::new( 3, 9);
        let f = Vertex::new(-2, 7);
        let vertices = vec![&a, &b, &c, &d, &e, &f];
        let polygon = Polygon::new(vertices);

        // TODO obviously this is a pain, should think more about line
        // segment and whether it's going to be a pain to have as a
        // primitive, the alternative could be to just use ordered
        // vertices (or assume order on vertices when input to funcs)
        let ac = LineSegment::new(&a, &c);
        let ad = LineSegment::new(&a, &d);
        let ae = LineSegment::new(&a, &e);
        let bd = LineSegment::new(&b, &d);
        let be = LineSegment::new(&b, &e);
        let bf = LineSegment::new(&b, &f);
        let ca = LineSegment::new(&c, &a);
        let ce = LineSegment::new(&c, &e);
        let cf = LineSegment::new(&c, &f);
        let da = LineSegment::new(&d, &a);
        let db = LineSegment::new(&d, &b);
        let df = LineSegment::new(&d, &f);
        let ea = LineSegment::new(&e, &a);
        let eb = LineSegment::new(&e, &b);
        let ec = LineSegment::new(&e, &c);
        let fb = LineSegment::new(&f, &b);
        let fc = LineSegment::new(&f, &c);
        let fd = LineSegment::new(&f, &d);

        let internal = vec![&ae, &bd, &be, &bf, &ce, &db, &df, &ea, &eb, &ec, &fb, &fd];
        let external = vec![&ac, &ca];
        let not_diagonal = vec![&ad, &cf, &da, &fc];
        
        for ls in internal {
            assert!(polygon.in_cone(ls));
            assert!(polygon.diagonal_internal_external(ls));
            assert!(polygon.diagonal(ls));
        }

        for ls in external {
            // TODO might want to think of another example and think carefully
            // about the in_cone, I think there'd be examples where at least
            // one of the directions fails
            assert!(!polygon.in_cone(ls));
            assert!( polygon.diagonal_internal_external(ls));
            assert!(!polygon.diagonal(ls));
        }

        for ls in not_diagonal {
            assert!(!polygon.diagonal_internal_external(ls));
            assert!(!polygon.diagonal(ls));
        }
    }
}
