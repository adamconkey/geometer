use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;


use crate::{
    line_segment::LineSegment,
    triangle::Triangle,
    vertex::Vertex,
};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Polygon {
    vertex_map: IndexMap<String, Vertex>,
}


impl Polygon {
    pub fn new(vertices: Vec<Vertex>) -> Polygon {
        let mut vertex_map = IndexMap::new();
        for vertex in vertices {
            vertex_map.insert(vertex.id.clone(), vertex);
        }

        Polygon { vertex_map }
    }

    pub fn from_json<P: AsRef<Path>>(path: P) -> Polygon {
        let polygon_str: String = fs::read_to_string(path)
            .expect("file should exist and be parseable");
        // TODO don't unwrap
        serde_json::from_str(&polygon_str).unwrap()
    }
    
    pub fn double_area(&self) -> i32 {
        // The first pair will include the anchor, but that area
        // ends up being zero since it will trivially be collinear
        // with anchor and thus doesn't affect the compuation
        let mut area = 0;
        let anchor = self.vertex_map.first().expect("should have >0 elements").1;
        for v1 in self.vertex_map.values() {
            let v2 = self.vertex_map.get(&v1.next).expect("Neighbor should be in vmap"); 
            area += Triangle::new(anchor, v1, v2).double_area();
        }
        area
    }

    pub fn triangulation(&self) -> Vec<LineSegment> {
        let mut triangulation = Vec::new();
        let mut vmap = self.vertex_map.clone();
        let mut anchor = vmap.first().expect("Should be >0 vertices").1;

        while vmap.len() > 3 {
            let mut v2 = anchor;
            
            loop {

                // TODO need to figure out how you can do this without
                // borrow errors, I'm mixing mutable/immutable here

                let v1 = vmap.get_mut(&v2.prev).unwrap();
                let v3 = vmap.get_mut(&v2.next).unwrap();

                if self.diagonal(&LineSegment::new(v1, v3)) {
                    // We found v2 is an ear vertex, add diagonal to the triangulation
                    triangulation.push(LineSegment::new(v1, v3));

                    let v4 = vmap.get(&v3.next).unwrap();
                    let v0 = vmap.get(&v1.prev).unwrap();
                    
                    // Remove ear vertex, and update its neighbors so that 
                    // their neighbors point to the correct vertices
                    vmap.shift_remove(&v2.id);
                    v1.prev = v0.id; 
                    v1.next = v3.id;
                    v3.prev = v1.id;
                    v3.next = v4.id;
                    anchor = v3;  // In case removed was anchor
                }

                v2 = v3;  // Advance to next vertex in chain

                if v2 == anchor {
                    // Made a full pass through all vertices, can advance to
                    // next iter of outer loop
                    break;
                }
            }
        }

        triangulation
    }

    pub fn get_vertex(&self, id: &str) -> Option<&Vertex> {
        self.vertex_map.get(id)
    }

    pub fn get_line_segment(&self, id_1: &str, id_2: &str) -> LineSegment {
        // TODO this should return Option<LineSegment> and handle 
        // the cases below instead of unwrap here
        let v1 = self.get_vertex(id_1).unwrap();
        let v2 = self.get_vertex(id_2).unwrap();
        LineSegment::new(v1, v2)
    }

    pub fn edges(&self) -> Vec<LineSegment> {
        // TODO could cache this and clear on modification
        let mut edges = Vec::new();
        for v in self.vertex_map.values() {
            let next = self.get_vertex(&v.next).unwrap();
            edges.push(LineSegment::new(v, next));
        }
        edges
    }
    
    pub fn in_cone(&self, ab: &LineSegment) -> bool {
        let a = ab.v1;
        let ba = &ab.reverse();
        // TODO do better than unwrap, prev and next should be optional
        let a0 = self.get_vertex(&a.prev).unwrap();
        let a1 = self.get_vertex(&a.next).unwrap();

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
    use std::path::PathBuf;

    fn load_polygon(filename: &str) -> Polygon {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        path.push(filename);
        Polygon::from_json(path)
    }

    // TODO these fixtures are nice and compact now, but it has me
    // wondering if this is the best way to support fixtures? Might
    // get tedious specifying these as the test cases grow. Could
    // just parameterize on filenames and then load vmap in the
    // the test
    #[fixture]
    fn polygon_1() -> Polygon {
        load_polygon("polygon_1.json")
    }

    #[fixture]
    fn polygon_2() -> Polygon {
        load_polygon("polygon_2.json")
    }

    #[fixture]
    fn right_triangle() -> Polygon {
        load_polygon("right_triangle.json")
    }

    #[fixture]
    fn square_4x4() -> Polygon {
        load_polygon("square_4x4.json")
    }


    #[rstest]
    // TODO now that this is parametrized, can add as many polygons
    // here as possible to get meaningful tests on area
    #[case(right_triangle(), 12)]
    #[case(polygon_2(), 466)]
    fn test_area(#[case] polygon: Polygon, #[case] expected_double_area: i32) {
        let double_area = polygon.double_area();
        assert_eq!(double_area, expected_double_area);
    }

    #[rstest]
    fn test_edges_square(square_4x4: Polygon) {
        let expected_edges = vec![
            square_4x4.get_line_segment("a", "b"),
            square_4x4.get_line_segment("b", "c"),
            square_4x4.get_line_segment("c", "d"),
            square_4x4.get_line_segment("d", "a"),
        ];
        assert_eq!(square_4x4.edges(), expected_edges);
    }
    
    #[rstest]
    fn test_edges_p1(polygon_1: Polygon) {
        let expected_edges = vec![
            polygon_1.get_line_segment("a", "b"),
            polygon_1.get_line_segment("b", "c"),
            polygon_1.get_line_segment("c", "d"),
            polygon_1.get_line_segment("d", "e"),
            polygon_1.get_line_segment("e", "f"),
            polygon_1.get_line_segment("f", "a"),
        ];
        assert_eq!(polygon_1.edges(), expected_edges);
    }

    #[rstest]
    fn test_diagonal(polygon_1: Polygon) {
        let ac = polygon_1.get_line_segment("a", "c");
        let ad = polygon_1.get_line_segment("a", "d");
        let ae = polygon_1.get_line_segment("a", "e");
        let bd = polygon_1.get_line_segment("b", "d");
        let be = polygon_1.get_line_segment("b", "e");
        let bf = polygon_1.get_line_segment("b", "f");
        let ca = polygon_1.get_line_segment("c", "a");
        let ce = polygon_1.get_line_segment("c", "e");
        let cf = polygon_1.get_line_segment("c", "f");
        let da = polygon_1.get_line_segment("d", "a");
        let db = polygon_1.get_line_segment("d", "b");
        let df = polygon_1.get_line_segment("d", "f");
        let ea = polygon_1.get_line_segment("e", "a");
        let eb = polygon_1.get_line_segment("e", "b");
        let ec = polygon_1.get_line_segment("e", "c");
        let fb = polygon_1.get_line_segment("f", "b");
        let fc = polygon_1.get_line_segment("f", "c");
        let fd = polygon_1.get_line_segment("f", "d");

        let internal = vec![&ae, &bd, &be, &bf, &ce, &db, &df, &ea, &eb, &ec, &fb, &fd];
        let external = vec![&ac, &ca];
        let not_diagonal = vec![&ad, &cf, &da, &fc];
        
        for ls in internal {
            assert!(polygon_1.in_cone(ls));
            assert!(polygon_1.diagonal_internal_external(ls));
            assert!(polygon_1.diagonal(ls));
        }

        for ls in external {
            // TODO might want to think of another example and think carefully
            // about the in_cone, I think there'd be examples where at least
            // one of the directions fails
            assert!(!polygon_1.in_cone(ls));
            assert!( polygon_1.diagonal_internal_external(ls));
            assert!(!polygon_1.diagonal(ls));
        }

        for ls in not_diagonal {
            assert!(!polygon_1.diagonal_internal_external(ls));
            assert!(!polygon_1.diagonal(ls));
        }
    }

    #[rstest]
    fn test_triangulation(polygon_2: Polygon) {
        let triangulation = polygon_2.triangulation();
        
        let ls_17_1 = polygon_2.get_line_segment("17", "1");
        let ls_1_3 = polygon_2.get_line_segment("1", "3");
        let ls_4_6 = polygon_2.get_line_segment("4", "6");
        let ls_4_7 = polygon_2.get_line_segment("4", "7");
        let ls_9_11 = polygon_2.get_line_segment("9", "11");
        let ls_12_14 = polygon_2.get_line_segment("12", "14");
        let ls_15_17 = polygon_2.get_line_segment("15", "17");
        let ls_15_1 = polygon_2.get_line_segment("15", "1");
        let ls_15_3 = polygon_2.get_line_segment("15", "3");
        let ls_3_7 = polygon_2.get_line_segment("3", "7");
        let ls_11_14 = polygon_2.get_line_segment("11", "14");
        let ls_15_7 = polygon_2.get_line_segment("15", "7");
        let ls_15_8 = polygon_2.get_line_segment("15", "8");
        let ls_15_9 = polygon_2.get_line_segment("15", "9");
        let ls_9_14 = polygon_2.get_line_segment("9", "14");
       
        let expected = vec![
            ls_17_1,
            ls_1_3,
            ls_4_6,
            ls_4_7,
            ls_9_11,
            ls_12_14,
            ls_15_17,
            ls_15_1,
            ls_15_3,
            ls_3_7,
            ls_11_14,
            ls_15_7,
            ls_15_8,
            ls_15_9,
            ls_9_14
        ];
        assert_eq!(expected, triangulation);
    }
}
