use std::collections::{hash_map, HashMap};
use std::fs;
use std::path::Path;


use crate::{
    line_segment::LineSegment,
    point::Point,
    triangle::Triangle,
    vertex::{Vertex, VertexId},
};

#[derive(Debug, PartialEq)]
pub struct Polygon {
    vertex_map: VertexMap,
}


// TODO now that this is abstracted out like this, it makes me realize it
// would make good sense to define a newtype wrapper around hashmap that
// encapsulates this. It's also making me rethink vertex a bit. It's been
// clumsy that I have to have the prev/next Option types, when really for
// a polygon in the way I'm approaching it, those should _always_ be
// defined for a valid polygon, you'll never have dangling vertices. So
// perhaps it will make good sense then to not pass in a vec of vertices
// to Polygon::new, but instead just points that store the x/y coordinates.
// Then as you iterate over them, you can create Vertex instances that have
// non-Option type prev/next, and an ID. The Point types could also have a
// name which could be propagated to the vertex, so that it could serve as
// an alias for look-up, which will be useful for tests/visualizations.
// I think this solves most of my problems I've had, it gets rid of Option
// so you don't have to unwrap a bunch of stuff, and the vmap operations
// can enforce that prev/next are always defined and are valid IDs in the
// map, so internally it could unwrap() on get operations while still
// offering a public lookup that could optional.


#[derive(Clone, Debug, PartialEq)]
pub struct VertexMap(HashMap<VertexId, Vertex>);


impl VertexMap {
    pub fn new(points: Vec<Point>) -> Self {
        let mut vertex_map = HashMap::new();

        let num_points = points.len();
        let vertex_ids = (0..num_points)
            .map(|_| VertexId::new(None))
            .collect::<Vec<_>>();

        for (i, point) in points.into_iter().enumerate() {
            let prev_id = vertex_ids[(i + num_points - 1) % num_points];
            let curr_id = vertex_ids[i];
            let next_id = vertex_ids[(i + num_points + 1) % num_points];
            let v = Vertex::new(point, curr_id, prev_id, next_id);
            vertex_map.insert(curr_id, v);
        }

        Self(vertex_map)
    }

    pub fn get(&self, k: &VertexId) -> &Vertex {
        // Unwrapping since this is for internal use only
        // and it will be assumed that internally we only 
        // operate on valid IDs in the map
        self.0.get(k).unwrap()
    }

    pub fn get_mut(&mut self, k: &VertexId) -> &mut Vertex{
        self.0.get_mut(k).unwrap()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn remove(&mut self, k: &VertexId) -> Vertex {
        self.0.remove(k).unwrap()
    }

    pub fn values(&self) -> hash_map::Values<'_, VertexId, Vertex> {
        self.0.values()
    }

    pub fn anchor(&self) -> &Vertex {
        // TODO I'm not yet convinced this is something I want, ultimately
        // need something to initiate algorithms in the vertex chain.
        // Could consider only exposing keys and then having polygon gen
        // an anchor
        self.values().collect::<Vec<_>>()[0]
    }

    pub fn update_next(&mut self, k: &VertexId, next: &VertexId) {
        self.get_mut(k).next = next.clone();
    }

    pub fn update_prev(&mut self, k: &VertexId, prev: &VertexId) {
        self.get_mut(k).prev = prev.clone();
    }
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
            area += Triangle::new(anchor, v1, v2).double_area();
        }
        area
    }

    pub fn triangulation(&self) -> Vec<(VertexId, VertexId)> {
        let mut triangulation = Vec::new();
        let mut vmap = self.vertex_map.clone();

        while vmap.len() > 3 {
            if let Some(v2_key) = self.find_ear(&vmap) {
                let v2 = vmap.remove(&v2_key);
                triangulation.push((v2.prev, v2.next));                
                vmap.update_next(&v2.prev, &v2.next);
                vmap.update_prev(&v2.next, &v2.prev);
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
            if self.diagonal(&self.get_line_segment(&v2.prev, &v2.next)) {
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
        LineSegment::new(v1, v2)
    }

    pub fn edges(&self) -> Vec<LineSegment> {
        // TODO could cache this and clear on modification
        let mut edges = Vec::new();
        let anchor_id = self.vertex_map.anchor().id;
        let mut current = self.get_vertex(&anchor_id);
        loop {
            edges.push(self.get_line_segment(&current.id, &current.next));
            current = self.get_vertex(&current.next);
            if current.id == anchor_id {
                break;
            }
        }
        edges
    }
    
    fn in_cone(&self, ab: &LineSegment) -> bool {
        let a = ab.v1;
        let ba = &ab.reverse();
        // TODO do better than unwrap, prev and next should be optional
        let a0 = self.get_vertex(&a.prev);
        let a1 = self.get_vertex(&a.next);

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
    //     let expected_edges = vec![
    //         polygon_1.get_line_segment("a", "b"),
    //         polygon_1.get_line_segment("b", "c"),
    //         polygon_1.get_line_segment("c", "d"),
    //         polygon_1.get_line_segment("d", "e"),
    //         polygon_1.get_line_segment("e", "f"),
    //         polygon_1.get_line_segment("f", "a"),
    //     ];
    //     assert_eq!(polygon_1.edges(), expected_edges);
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
