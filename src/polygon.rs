use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};
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

    pub fn vertices(&self) -> Vec<&Vertex> {
        // TODO can rethink edges method if this works

        let mut vertices = Vec::new();
        let mut current = self.anchor;

        loop {
            vertices.push(current);
            current = self.neighbors
                .get(current)
                .expect("Every vertex should have neighbors stored")
                .1;
            
            if current == self.anchor {
                break;
            }
        }

        vertices
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

    pub fn triangulation(&self) -> Vec<LineSegment> {
        let mut triangulation = Vec::new();
        let mut neighbors = self.neighbors.clone();
        let mut anchor = self.anchor;

        while neighbors.len() > 3 {
            let mut v2 = anchor;
            
            loop {
                // Removing instead of borrowing, so that we don't run into 
                // immutable borrow problems. It will be inserted again if 
                // we end up not finding an ear. May want to rethink this 
                // if there's a better way to go about so we're not 
                // unnecessarily removing things.
                let (v1, v3) = neighbors
                    .remove(v2)
                    .expect("Every vertex should have neighbors stored");

                if self.diagonal(&LineSegment::new(v1, v3)) {
                    // We found an ear, add to the triangulation
                    triangulation.push(LineSegment::new(v1, v3));

                    let v4 = neighbors
                        .get(v3)
                        .expect("Every vertex should have neighbors stored")
                        .1;
                    let v0 = neighbors
                        .get(v1)
                        .expect("Every vertex should have neighbors stored")
                        .0;

                    // The ear vertex has been removed, update its neighbors 
                    // so that their neighbors point to the correct vertices
                    neighbors.insert(v1, (v0, v3));
                    neighbors.insert(v3, (v1, v4));
                    anchor = v3;  // In case removed was anchor
                }
                else {
                    // This wasn't an ear, so re-insert into neighbor map
                    neighbors.insert(v2, (v1, v3));
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


impl<'a> Serialize for Polygon<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let vertices = self.vertices();
        let mut seq = serializer.serialize_seq(Some(vertices.len()))?;
        for e in vertices {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use std::str::FromStr;

    // TODO I think it will be better to ultimately read these
    // from file since I'll likely have some with many vertices
    // which will get a little unwieldy here.
    #[fixture]
    fn polygon_1() -> &'static str {
        concat!("0 0 a\n", "3 4 b\n", "6 2 c\n", "7 6 d\n", "3 9 e\n", "-2 7 f")
    }

    #[fixture]
    fn polygon_2() -> &'static str {
        concat!(
            "0 0 0\n",
            "12 9 1\n",
            "14 4 2\n",
            "24 10 3\n",
            "14 24 4\n",
            "11 17 5\n",
            "13 19 6\n",
            "14 12 7\n",
            "9 13 8\n",
            "7 18 9\n",
            "11 21 10\n",
            "8 24 11\n",
            "1 22 12\n",
            "2 18 13\n",
            "4 20 14\n",
            "6 10 15\n",
            "-2 11 16\n",
            "6 6 17"
        )
    }

    #[fixture]
    fn right_triangle() -> &'static str {
        concat!("0 0 a\n", "3 0 b\n", "0 4 c")
    }

    #[fixture]
    fn square_4x4() -> &'static str {
        concat!("0 0 a\n", "4 0 b\n", "4 4 c\n", "0 4 d")
    }


    #[rstest]
    fn test_serialize_right_triangle(right_triangle: &str) {
        let vmap = VertexMap::from_str(right_triangle).unwrap();
        let polygon = Polygon::from_vmap(&vmap);
        let expected_serialized = r#"[{"x":0,"y":0,"id":"a"},{"x":3,"y":0,"id":"b"},{"x":0,"y":4,"id":"c"}]"#;
        let serialized = serde_json::to_string(&polygon).unwrap();
        // let deserialized: Polygon = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serialized, expected_serialized);
    }

    #[rstest]
    // TODO now that this is parametrized, can add as many polygons
    // here as possible to get meaningful tests on area
    #[case(right_triangle(), 12)]
    #[case(polygon_2(), 466)]
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
    fn test_edges_square(square_4x4: &str) {
        let vmap = VertexMap::from_str(square_4x4).unwrap();
        let polygon = Polygon::from_vmap(&vmap);

        let expected_edges = vec![
            vmap.get_line_segment("a", "b"),
            vmap.get_line_segment("b", "c"),
            vmap.get_line_segment("c", "d"),
            vmap.get_line_segment("d", "a"),
        ];
    
        assert_eq!(polygon.edges(), expected_edges);
    }

    #[rstest]
    fn test_vertices_square(square_4x4: &str) {
        let vmap = VertexMap::from_str(square_4x4).unwrap();
        let polygon = Polygon::from_vmap(&vmap);

        let expected_vertices = vec![
            vmap.get("a").unwrap(),
            vmap.get("b").unwrap(),
            vmap.get("c").unwrap(),
            vmap.get("d").unwrap(),
        ];
    
        assert_eq!(polygon.vertices(), expected_vertices);
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
    fn test_edges_p1(polygon_1: &str) {
        let vmap = VertexMap::from_str(polygon_1).unwrap();
        let polygon = Polygon::from_vmap(&vmap);

        let expected_edges = vec![
            vmap.get_line_segment("a", "b"),
            vmap.get_line_segment("b", "c"),
            vmap.get_line_segment("c", "d"),
            vmap.get_line_segment("d", "e"),
            vmap.get_line_segment("e", "f"),
            vmap.get_line_segment("f", "a"),
        ];
        
        assert_eq!(polygon.edges(), expected_edges);
    }

    #[rstest]
    fn test_vertices_p1(polygon_1: &str) {
        let vmap = VertexMap::from_str(polygon_1).unwrap();
        let polygon = Polygon::from_vmap(&vmap);

        let expected_vertices = vec![
            vmap.get("a").unwrap(),
            vmap.get("b").unwrap(),
            vmap.get("c").unwrap(),
            vmap.get("d").unwrap(),
            vmap.get("e").unwrap(),
            vmap.get("f").unwrap(),
        ];
        
        assert_eq!(polygon.vertices(), expected_vertices);
    }

    #[rstest]
    fn test_diagonal(polygon_1: &str) {
        let vmap = VertexMap::from_str(polygon_1).unwrap();
        let polygon = Polygon::from_vmap(&vmap);

        let ac = vmap.get_line_segment("a", "c");
        let ad = vmap.get_line_segment("a", "d");
        let ae = vmap.get_line_segment("a", "e");
        let bd = vmap.get_line_segment("b", "d");
        let be = vmap.get_line_segment("b", "e");
        let bf = vmap.get_line_segment("b", "f");
        let ca = vmap.get_line_segment("c", "a");
        let ce = vmap.get_line_segment("c", "e");
        let cf = vmap.get_line_segment("c", "f");
        let da = vmap.get_line_segment("d", "a");
        let db = vmap.get_line_segment("d", "b");
        let df = vmap.get_line_segment("d", "f");
        let ea = vmap.get_line_segment("e", "a");
        let eb = vmap.get_line_segment("e", "b");
        let ec = vmap.get_line_segment("e", "c");
        let fb = vmap.get_line_segment("f", "b");
        let fc = vmap.get_line_segment("f", "c");
        let fd = vmap.get_line_segment("f", "d");

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

    #[rstest]
    fn test_triangulation(polygon_2: &str) {
        let vmap = VertexMap::from_str(polygon_2).unwrap();        
        let polygon = Polygon::from_vmap(&vmap);
        let triangulation = polygon.triangulation();
        
        let ls_17_1 = vmap.get_line_segment("17", "1");
        let ls_1_3 = vmap.get_line_segment("1", "3");
        let ls_4_6 = vmap.get_line_segment("4", "6");
        let ls_4_7 = vmap.get_line_segment("4", "7");
        let ls_9_11 = vmap.get_line_segment("9", "11");
        let ls_12_14 = vmap.get_line_segment("12", "14");
        let ls_15_17 = vmap.get_line_segment("15", "17");
        let ls_15_1 = vmap.get_line_segment("15", "1");
        let ls_15_3 = vmap.get_line_segment("15", "3");
        let ls_3_7 = vmap.get_line_segment("3", "7");
        let ls_11_14 = vmap.get_line_segment("11", "14");
        let ls_15_7 = vmap.get_line_segment("15", "7");
        let ls_15_8 = vmap.get_line_segment("15", "8");
        let ls_15_9 = vmap.get_line_segment("15", "9");
        let ls_9_14 = vmap.get_line_segment("9", "14");
       
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
