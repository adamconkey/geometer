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

        Polygon { anchor: &vertices[0], neighbors }
    }

    pub fn from_vmap(vmap: &'a VertexMap) -> Polygon<'a> {
        Polygon::new(vmap.all_vertices())
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
    use rstest::{fixture, rstest};
    use std::str::FromStr;

    #[fixture]
    fn polygon_1() -> &'static str {
        "0 0 a\n3 4 b\n6 2 c\n7 6 d\n3 9 e\n-2 7 f"
    }

    #[fixture]
    fn right_triangle() -> &'static str {
        "0 0 a\n3 0 b\n0 4 c"
    }

    #[fixture]
    fn square_4x4() -> &'static str {
        "0 0 a\n4 0 b\n4 4 c\n0 4 d"
    }
    
    #[rstest]
    // TODO now that this is parametrized, can add as many polygons
    // here as possible to get meaningful tests on area
    #[case(right_triangle(), 12)]
    fn test_area(#[case] polygon_str: &str, #[case] expected_double_area: i32) {
        let vmap = VertexMap::from_str(polygon_str).unwrap();        
        let polygon = Polygon::from_vmap(&vmap);
        let double_area = polygon.double_area();
        assert_eq!(double_area, expected_double_area);
    }

    #[rstest]
    fn test_neighbors_square(square_4x4: &str) {
        let vmap = VertexMap::from_str(square_4x4).unwrap();
        let polygon = Polygon::from_vmap(&vmap);

        let a = vmap.get("a").unwrap();
        let b = vmap.get("b").unwrap();
        let c = vmap.get("c").unwrap();
        let d = vmap.get("d").unwrap();
        
        assert_eq!(polygon.neighbors(a), (d, b));
        assert_eq!(polygon.neighbors(b), (a, c));
        assert_eq!(polygon.neighbors(c), (b, d));
        assert_eq!(polygon.neighbors(d), (c, a));
    }

    #[rstest]
    fn test_neighbors_p1(polygon_1: &str) {
        let vmap = VertexMap::from_str(polygon_1).unwrap();
        let polygon = Polygon::from_vmap(&vmap);

        let a = vmap.get("a").unwrap();
        let b = vmap.get("b").unwrap();
        let c = vmap.get("c").unwrap();
        let d = vmap.get("d").unwrap();
        let e = vmap.get("e").unwrap();
        let f = vmap.get("f").unwrap();
        
        assert_eq!(polygon.neighbors(a), (f, b));
        assert_eq!(polygon.neighbors(b), (a, c));
        assert_eq!(polygon.neighbors(c), (b, d));
        assert_eq!(polygon.neighbors(d), (c, e));
        assert_eq!(polygon.neighbors(e), (d, f));
        assert_eq!(polygon.neighbors(f), (e, a));
    }
    
    #[rstest]
    fn test_diagonal(polygon_1: &str) {
        let vmap = VertexMap::from_str(polygon_1).unwrap();
        let polygon = Polygon::from_vmap(&vmap);

        let a = vmap.get("a").unwrap();
        let b = vmap.get("b").unwrap();
        let c = vmap.get("c").unwrap();
        let d = vmap.get("d").unwrap();
        let e = vmap.get("e").unwrap();
        let f = vmap.get("f").unwrap();
    
        let ac = LineSegment::new(a, c);
        let ad = LineSegment::new(a, d);
        let ae = LineSegment::new(a, e);
        let bd = LineSegment::new(b, d);
        let be = LineSegment::new(b, e);
        let bf = LineSegment::new(b, f);
        let ca = LineSegment::new(c, a);
        let ce = LineSegment::new(c, e);
        let cf = LineSegment::new(c, f);
        let da = LineSegment::new(d, a);
        let db = LineSegment::new(d, b);
        let df = LineSegment::new(d, f);
        let ea = LineSegment::new(e, a);
        let eb = LineSegment::new(e, b);
        let ec = LineSegment::new(e, c);
        let fb = LineSegment::new(f, b);
        let fc = LineSegment::new(f, c);
        let fd = LineSegment::new(f, d);

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
