use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;


use crate::{
    line_segment::LineSegment,
    triangle::Triangle,
    vertex::{Vertex, VertexId},
};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Polygon {
    vertex_map: HashMap<VertexId, Vertex>,
    anchor: VertexId,
}


impl Polygon {
    pub fn new(vertices: Vec<Vertex>) -> Polygon {
        let mut vertex_map = HashMap::new();
        let first_id = vertices.first().unwrap().id;
        let last_id = vertices.last().unwrap().id;

        // TODO need to populate prev/next here while adding the vertices
        // to the map, otherwise they won't be registered so structure
        // isn't represented later.

        for (v0, v1, v2) in vertices.iter().tuple_windows::<(_,_,_)>() {

            let mut v = v1.clone();
            v.prev = Some(v0.id);
            v.next = Some(v2.id);
            vertex_map.insert(v.id, v);

            if v0.id == first_id {
                let mut first = v0.clone();
                first.prev = Some(last_id);
                first.next = Some(v1.id);
                vertex_map.insert(first.id, first);
            }
            if v2.id == last_id {
                let mut last = v2.clone();
                last.prev = Some(v1.id);
                last.next = Some(first_id);
                vertex_map.insert(last.id, last);
            }
            
        }
        Polygon { vertex_map, anchor: first_id }
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
        let anchor = self.get_vertex(&self.anchor).unwrap();
        for v1 in self.vertex_map.values() {
            let v2 = self.get_vertex(&v1.next.unwrap()).unwrap(); 
            area += Triangle::new(anchor, v1, v2).double_area();
        }
        area
    }

    pub fn triangulation(&self) -> Vec<(VertexId, VertexId)> {
        let mut triangulation = Vec::new();
        let mut vmap = self.vertex_map.clone();

        while vmap.len() > 3 {
            if let Some(v2_key) = self.find_ear(&vmap) {
                let v2 = vmap.remove(&v2_key).unwrap();
                triangulation.push((v2.prev.unwrap(), v2.next.unwrap()));
                
                let v1 = vmap.get_mut(&v2.prev.unwrap()).unwrap();
                v1.next = v2.next;                
                let v3 = vmap.get_mut(&v2.next.unwrap()).unwrap();
                v3.prev = v2.prev;
            }
            else {
                panic!("BAD THINGS need to fix this")
                // TODO this is actually an error and shouldn't actually
                // happen if it's a valid polygon, so need to think of
                // how this case should be handled
            }
        }
        triangulation
    }

    pub fn find_ear(&self, vmap: &HashMap<VertexId, Vertex>) -> Option<VertexId> {
        for v2 in vmap.values() {
            let v1 = vmap.get(&v2.prev.unwrap()).unwrap();
            let v3 = vmap.get(&v2.next.unwrap()).unwrap();
            if self.diagonal(&self.get_line_segment(&v1.id, &v3.id)) {
                return Some(v2.id);
            }
        }
        None
    }

    pub fn get_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        self.vertex_map.get(id)
    }

    pub fn get_line_segment(&self, id_1: &VertexId, id_2: &VertexId) -> LineSegment {
        // TODO this should return Option<LineSegment> and handle 
        // the cases below instead of unwrap here
        let v1 = self.get_vertex(id_1).unwrap();
        let v2 = self.get_vertex(id_2).unwrap();
        LineSegment::new(v1, v2)
    }

    pub fn edges(&self) -> Vec<LineSegment> {
        // TODO could cache this and clear on modification
        let mut edges = Vec::new();
        let mut current_id = &self.anchor;
        loop {
            let current = self.get_vertex(&current_id).unwrap();
            let next = self.get_vertex(&current.next.unwrap()).unwrap();
            edges.push(LineSegment::new(current, next));
            current_id = &next.id;
            if current_id == &self.anchor {
                break;
            }
        }
        edges
    }
    
    pub fn in_cone(&self, ab: &LineSegment) -> bool {
        let a = ab.v1;
        let ba = &ab.reverse();
        // TODO do better than unwrap, prev and next should be optional
        let a0 = self.get_vertex(&a.prev.unwrap()).unwrap();
        let a1 = self.get_vertex(&a.next.unwrap()).unwrap();

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
        // TODO manually creating until I fix file stuff
        // load_polygon("right_triangle.json")
        let v0 = Vertex::new(0, 0);
        let v1 = Vertex::new(3, 0);
        let v2 = Vertex::new(0, 4);
        let vertices = vec![v0, v1, v2];
        Polygon::new(vertices)
    }

    #[fixture]
    fn square_4x4() -> Polygon {
        load_polygon("square_4x4.json")
    }


    #[rstest]
    // TODO now that this is parametrized, can add as many polygons
    // here as possible to get meaningful tests on area
    #[case(right_triangle(), 12)]
    // #[case(polygon_2(), 466)]
    fn test_area(#[case] polygon: Polygon, #[case] expected_double_area: i32) {
        let double_area = polygon.double_area();
        assert_eq!(double_area, expected_double_area);
    }

    // #[rstest]
    // fn test_edges_square(square_4x4: Polygon) {
    //     // let expected_edges = vec![
    //     //     square_4x4.get_line_segment("a", "b"),
    //     //     square_4x4.get_line_segment("b", "c"),
    //     //     square_4x4.get_line_segment("c", "d"),
    //     //     square_4x4.get_line_segment("d", "a"),
    //     // ];
    //     // assert_eq!(square_4x4.edges(), expected_edges);
    // }
    
    // #[rstest]
    // fn test_edges_p1(polygon_1: Polygon) {
    //     // let expected_edges = vec![
    //     //     polygon_1.get_line_segment("a", "b"),
    //     //     polygon_1.get_line_segment("b", "c"),
    //     //     polygon_1.get_line_segment("c", "d"),
    //     //     polygon_1.get_line_segment("d", "e"),
    //     //     polygon_1.get_line_segment("e", "f"),
    //     //     polygon_1.get_line_segment("f", "a"),
    //     // ];
    //     // assert_eq!(polygon_1.edges(), expected_edges);
    // }

    // #[rstest]
    // fn test_diagonal(polygon_1: Polygon) {
    //     // let ac = polygon_1.get_line_segment("a", "c");
    //     // let ad = polygon_1.get_line_segment("a", "d");
    //     // let ae = polygon_1.get_line_segment("a", "e");
    //     // let bd = polygon_1.get_line_segment("b", "d");
    //     // let be = polygon_1.get_line_segment("b", "e");
    //     // let bf = polygon_1.get_line_segment("b", "f");
    //     // let ca = polygon_1.get_line_segment("c", "a");
    //     // let ce = polygon_1.get_line_segment("c", "e");
    //     // let cf = polygon_1.get_line_segment("c", "f");
    //     // let da = polygon_1.get_line_segment("d", "a");
    //     // let db = polygon_1.get_line_segment("d", "b");
    //     // let df = polygon_1.get_line_segment("d", "f");
    //     // let ea = polygon_1.get_line_segment("e", "a");
    //     // let eb = polygon_1.get_line_segment("e", "b");
    //     // let ec = polygon_1.get_line_segment("e", "c");
    //     // let fb = polygon_1.get_line_segment("f", "b");
    //     // let fc = polygon_1.get_line_segment("f", "c");
    //     // let fd = polygon_1.get_line_segment("f", "d");

    //     // let internal = vec![&ae, &bd, &be, &bf, &ce, &db, &df, &ea, &eb, &ec, &fb, &fd];
    //     // let external = vec![&ac, &ca];
    //     // let not_diagonal = vec![&ad, &cf, &da, &fc];
        
    //     // for ls in internal {
    //     //     assert!(polygon_1.in_cone(ls));
    //     //     assert!(polygon_1.diagonal_internal_external(ls));
    //     //     assert!(polygon_1.diagonal(ls));
    //     // }

    //     // for ls in external {
    //     //     // TODO might want to think of another example and think carefully
    //     //     // about the in_cone, I think there'd be examples where at least
    //     //     // one of the directions fails
    //     //     assert!(!polygon_1.in_cone(ls));
    //     //     assert!( polygon_1.diagonal_internal_external(ls));
    //     //     assert!(!polygon_1.diagonal(ls));
    //     // }

    //     // for ls in not_diagonal {
    //     //     assert!(!polygon_1.diagonal_internal_external(ls));
    //     //     assert!(!polygon_1.diagonal(ls));
    //     // }
    // }

    // #[rstest]
    // fn test_triangulation(polygon_2: Polygon) {
    //     let triangulation = polygon_2.triangulation();

    //     // TODO will need to generally have better ways to assert
    //     // on triangulations. Currently my implementation is a bit
    //     // unstable and so not only can the order of line segments
    //     // be different, you can get slightly different triangulations
    //     // depending on the order things are visited. So it's 
    //     // probably better to just have some asserts that show it's
    //     // a valid triangulation, and then eventually if it's
    //     // stable enough can do an assert on the specific
    //     // triangulation achieved

    //     assert_eq!(triangulation.len(), 15);
    // }
}
