use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::{
    line_segment::LineSegment,
    point::Point,
    trapezoidization::Trapezoidization,
    triangle::Triangle,
    triangulation::{EarNotFoundError, TriangleVertexIds, Triangulation},
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
            if self.diagonal(&self.get_vertex(&v.prev), self.get_vertex(&v.next)) {
                return Ok(v.id);
            }
        }
        Err(EarNotFoundError)
    }

    pub fn trapezoidization(&self) -> Trapezoidization {

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
    fn test_edges(#[case] polygon: Polygon, #[case] num_edges: usize) {
        let mut expected_edges = HashSet::new();
        for i in 0usize..num_edges {
            expected_edges.insert((VertexId::from(i), VertexId::from((i + 1) % num_edges)));
        }
        let edges = polygon.edges();
        assert_eq!(edges, expected_edges);
    }

    #[rstest]
    #[case(right_triangle(), 1, 12)]
    #[case(square_4x4(), 2, 32)]
    #[case(polygon_2(), 16, 466)]
    fn test_triangulation(
        #[case] polygon: Polygon, 
        #[case] expected_num_triangles: usize, 
        #[case] expected_double_area: i32,
    ) {
        let triangulation = polygon.triangulation();
        assert_eq!(triangulation.len(), expected_num_triangles);

        let triangulation_double_area = polygon.double_area_from_triangulation(&triangulation);
        assert_eq!(triangulation_double_area, expected_double_area);
    }
}