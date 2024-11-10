use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::{
    line_segment::LineSegment,
    point::Point,
    triangle::Triangle,
    vertex::{Vertex, VertexId},
    vertex_map::VertexMap,
};


#[derive(Debug)]
pub struct Polygon {
    vertex_map: VertexMap,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        let vertex_map = VertexMap::new(points);
        Polygon { vertex_map }
    }

    pub fn from_json<P: AsRef<Path>>(path: P) -> Polygon {
        let polygon_str: String = fs::read_to_string(path)
            .expect("file should exist and be parseable");
        // TODO don't unwrap
        let points: Vec<Point> = serde_json::from_str(&polygon_str).unwrap();
        Polygon::new(points)
    }
    
    pub fn double_area(&self) -> i32 {
        let mut area = 0;
        let anchor = self.vertex_map.anchor();
        for v1 in self.vertex_map.values() {
            let v2 = self.get_vertex(&v1.next); 
            area += Triangle::from_vertices(anchor, v1, v2).double_area();
        }
        area
    }

    pub fn triangulation(&self) -> HashSet<(VertexId, VertexId)> {
        let mut triangulation = HashSet::new();
        let mut vmap = self.vertex_map.clone();

        while vmap.len() > 3 {
            if let Some(v2_key) = self.find_ear(&vmap) {
                let v2 = vmap.remove(&v2_key);
                triangulation.insert((v2.prev, v2.next));
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

    fn find_ear(&self, vmap: &VertexMap) -> Option<VertexId> {
        // TODO I think this should return Result instead of option
        // because really it should always return an ear on N>=3
        // vertices if it's a valid polygon, which the assumption
        // should be internally it is a valid polygon after creation
        // conserved by all internal ops on it. So if this returns
        // result then can just do handling of that in triangulation
        // and you won't have this awkware if else panic block
        for v2 in vmap.values() {
            if self.diagonal(&self.get_vertex(&v2.prev), self.get_vertex(&v2.next)) {
                return Some(v2.id);
            }
        }
        None
    }

    fn get_vertex(&self, id: &VertexId) -> &Vertex {
        self.vertex_map.get(id)
    }

    fn get_line_segment(&self, id_1: &VertexId, id_2: &VertexId) -> LineSegment {
        let v1 = self.get_vertex(id_1);
        let v2 = self.get_vertex(id_2);
        LineSegment::from_vertices(v1, v2)
    }

    pub fn edges(&self) -> HashSet<(VertexId, VertexId)> {
        // TODO could cache this and clear on modification
        let mut edges = HashSet::new();
        let anchor_id = self.vertex_map.anchor().id;
        let mut current = self.get_vertex(&anchor_id);
        loop {
            edges.insert((current.id, current.next));
            current = self.get_vertex(&current.next);
            if current.id == anchor_id {
                break;
            }
        }
        edges
    }
    
    fn in_cone(&self, a: &Vertex, b: &Vertex) -> bool {
        let ab = LineSegment::from_vertices(a, b);
        let ba = &ab.reverse();
        // TODO do better than unwrap, prev and next should be optional
        let a0 = self.get_vertex(&a.prev);
        let a1 = self.get_vertex(&a.next);

        if a0.left_on(&LineSegment::from_vertices(a, a1)) {
            return a0.left(&ab) && a1.left(ba);
        }
        
        // Otherwise a is reflexive
        !(a1.left_on(&ab) && a0.left_on(ba))
    }
    
    pub fn diagonal(&self, a: &Vertex, b: &Vertex) -> bool {
        self.in_cone(a, b) && self.in_cone(b, a) && self.diagonal_internal_external(a, b)
    }

    fn diagonal_internal_external(&self, a: &Vertex, b: &Vertex) -> bool {
        let ab = &LineSegment::from_vertices(a, b);
        for (id1, id2) in self.edges() {
            let e = self.get_line_segment(&id1, &id2);
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
    #[case(square_4x4(), 32)]
    #[case(polygon_2(), 466)]
    fn test_area(#[case] polygon: Polygon, #[case] expected_double_area: i32) {
        let double_area = polygon.double_area();
        assert_eq!(double_area, expected_double_area);
    }

    #[rstest]
    #[case(right_triangle(), 3)]
    #[case(square_4x4(), 4)]
    #[case(polygon_1(), 6)]
    #[case(polygon_2(), 18)]
    fn test_edges_square(#[case] polygon: Polygon, #[case] num_edges: usize) {
        let mut expected_edges = HashSet::new();
        // TODO 
        // A more interesting test would be removing vertices and
        // checking updates. But this one can be parameterized and
        // determined by num vertices, subsume test below
        for i in 0usize..num_edges {
            expected_edges.insert((VertexId::from(i), VertexId::from((i + 1) % num_edges)));
        }

        let edges = polygon.edges();
        assert_eq!(edges, expected_edges);
    }

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

    #[rstest]
    fn test_triangulation(polygon_2: Polygon) {
        let triangulation = polygon_2.triangulation();

        // TODO will need to generally have better ways to assert
        // on triangulations. Currently my implementation is a bit
        // unstable and so not only can the order of line segments
        // be different, you can get slightly different triangulations
        // depending on the order things are visited. So it's 
        // probably better to just have some asserts that show it's
        // a valid triangulation, and then eventually if it's
        // stable enough can do an assert on the specific
        // triangulation achieved

        assert_eq!(triangulation.len(), 15);
    }
}
