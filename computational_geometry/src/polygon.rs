use core::fmt;
use std::collections::HashSet;
use std::collections::hash_set::Iter;
use std::fs;
use std::path::Path;

use crate::{
    line_segment::LineSegment,
    point::Point,
    triangle::Triangle,
    vertex::{Vertex, VertexId},
    vertex_map::VertexMap,
};


#[derive(Debug, Clone)]
struct EarNotFoundError;

impl fmt::Display for EarNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "polygon is likely invalid")
    }
}


#[derive(Eq, Hash, PartialEq)]
pub struct TriangleVertexIds(VertexId, VertexId, VertexId);


pub struct Triangulation<'a> {
    triangles: HashSet<TriangleVertexIds>,
    vmap: &'a VertexMap,
}

impl<'a> Triangulation<'a> {
    pub fn new(vmap: &'a VertexMap) -> Self {
        Self { triangles: HashSet::new(), vmap }
    }    
    
    pub fn insert(&mut self, value: TriangleVertexIds) -> bool {
        self.triangles.insert(value)
    }

    pub fn iter(&self) -> Iter<'_, TriangleVertexIds> { 
        self.triangles.iter()
    }

    pub fn len(&self) -> usize {
        self.triangles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.triangles.is_empty()
    }

    pub fn to_points(&self) -> HashSet<(Point, Point, Point)> {
        self.triangles.iter()
            .map(|ids| 
                (
                    self.vmap.get(&ids.0).coords.clone(),
                    self.vmap.get(&ids.1).coords.clone(),
                    self.vmap.get(&ids.2).coords.clone()
                )
            ).collect()        
    }
}


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
        // Computes double area of the polygon.
        //
        // NOTE: This is computing double area so that I can stick to
        // i32 return types in this preliminary implementation and
        // not worry about floating point issues.
        let mut area = 0;
        let anchor = self.vertex_map.anchor();
        for v1 in self.vertex_map.values() {
            let v2 = self.get_vertex(&v1.next); 
            area += Triangle::from_vertices(anchor, v1, v2).double_area();
        }
        area
    }

    pub fn double_area_from_triangulation(&self, triangulation: &Triangulation) -> i32 {
        // Computes double area from a triangulation as the sum of
        // the double area of the individual triangles that 
        // constitute the triangulation.
        // 
        // This value should always be exactly equal to `self.double_area()`.
        let mut area = 0;
        for (p1, p2, p3) in triangulation.to_points().iter() {
            area += Triangle::new(p1, p2, p3).double_area();
        }
        area
    }

    pub fn triangulation(&self) -> Triangulation {
        let mut triangulation = Triangulation::new(&self.vertex_map);
        let mut vmap = self.vertex_map.clone();

        while vmap.len() > 3 {
            let id = self.find_ear(&vmap)
                .expect("valid polygons with 3 or more vertices should have an ear");
            let v = vmap.remove(&id);
            triangulation.insert(TriangleVertexIds(v.prev, id, v.next));
        }
        // At this stage there should be exactly 3 vertices left,
        // which form the final triangle of the triangulation
        let v = vmap.anchor();
        triangulation.insert(TriangleVertexIds(v.prev, v.id, v.next));

        triangulation
    }

    fn find_ear(&self, vmap: &VertexMap) -> Result<VertexId, EarNotFoundError> {
        for v in vmap.values() {
            if self.diagonal(self.get_vertex(&v.prev), self.get_vertex(&v.next)) {
                return Ok(v.id);
            }
        }
        Err(EarNotFoundError)
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
    use rstest_reuse::{self, *};
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Deserialize)]
    struct PolygonMetadata {
        double_area: i32,
        num_edges: usize,
        num_triangles: usize,
    }

    struct PolygonTestCase {
        polygon: Polygon,
        metadata: PolygonMetadata,
    }

    impl PolygonTestCase {
        fn new(polygon: Polygon, metadata: PolygonMetadata) -> Self {
            PolygonTestCase { polygon, metadata }
        }
    }

    fn load_polygon(name: &str, folder: &str) -> Polygon {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        path.push(folder);
        path.push(format!("{}.json", name));
        Polygon::from_json(path)
    }

    fn load_metadata(name: &str, folder: &str) -> PolygonMetadata {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        path.push(folder);
        path.push(format!("{}.meta.json", name));
        let metadata_str: String = fs::read_to_string(path)
            .expect("file should exist and be parseable");
        serde_json::from_str(&metadata_str).unwrap()
    }

    #[macro_export]
    macro_rules! polygon_fixture {
        ($name:ident, $folder:expr) => {
            #[fixture]
            fn $name() -> PolygonTestCase {
                PolygonTestCase::new(
                    load_polygon(stringify!($name), stringify!($folder)),
                    load_metadata(stringify!($name), stringify!($folder))
                )
            }
        };
    }

    polygon_fixture!(polygon_1, custom);
    polygon_fixture!(polygon_2, custom);
    polygon_fixture!(right_triangle, custom);
    polygon_fixture!(square_4x4, custom);

    polygon_fixture!(elgindy_1, interesting_polygon_archive);
    polygon_fixture!(gray_embroidery, interesting_polygon_archive);
    polygon_fixture!(held_12, interesting_polygon_archive);
    polygon_fixture!(held_7a, interesting_polygon_archive);
    polygon_fixture!(held_7b, interesting_polygon_archive);
    polygon_fixture!(held_7c, interesting_polygon_archive);
    polygon_fixture!(held_7d, interesting_polygon_archive);
    polygon_fixture!(mapbox_building, interesting_polygon_archive);
    polygon_fixture!(matisse_nuit, interesting_polygon_archive);

    #[template]
    #[rstest]
    #[case::right_triangle(right_triangle())]
    #[case::square_4x4(square_4x4())]
    #[case::polygon_1(polygon_1())]
    #[case::polygon_2(polygon_2())]
    #[case::elgindy_1(elgindy_1())]
    #[case::gray_embroidery(gray_embroidery())]
    #[case::held_12(held_12())]
    #[case::held_7a(held_7a())]
    #[case::held_7b(held_7b())]
    #[case::held_7c(held_7c())]
    #[case::held_7d(held_7d())]
    #[case::mapbox_building(mapbox_building())]
    #[case::matisse_nuit(matisse_nuit())]
    fn all_polygons(#[case] case: PolygonTestCase) {}


    #[apply(all_polygons)]
    fn test_area(case: PolygonTestCase) {
        let double_area = case.polygon.double_area();
        assert_eq!(double_area, case.metadata.double_area);
    }

    #[apply(all_polygons)]
    fn test_edges(case: PolygonTestCase) {
        let mut expected_edges = HashSet::new();
        for i in 0usize..case.metadata.num_edges {
            let src = VertexId::from(i);
            let dest = VertexId::from((i + 1) % case.metadata.num_edges);
            expected_edges.insert((src, dest));
        }
        let edges = case.polygon.edges();
        assert_eq!(edges.len(), case.metadata.num_edges);
        assert_eq!(edges, expected_edges);
    }

    #[apply(all_polygons)]
    fn test_triangulation(case: PolygonTestCase) {
        let triangulation = case.polygon.triangulation();
        assert_eq!(triangulation.len(), case.metadata.num_triangles);

        let triangulation_double_area = case.polygon.double_area_from_triangulation(&triangulation);
        assert_eq!(triangulation_double_area, case.metadata.double_area);
    }
}