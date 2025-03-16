use itertools::Itertools;
use ordered_float::OrderedFloat as OF;
use serde::Deserialize;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::geometry::Geometry;
use crate::{
    bounding_box::BoundingBox,
    error::FileError,
    line_segment::LineSegment,
    triangle::Triangle,
    vertex::{Vertex, VertexId},
};

#[derive(Deserialize)]
pub struct PolygonMetadata {
    pub area: f64,
    pub extreme_points: Vec<VertexId>,
    pub num_edges: usize,
    pub num_triangles: usize,
    pub num_vertices: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polygon {
    vertex_map: HashMap<VertexId, Vertex>,
    prev_map: HashMap<VertexId, VertexId>,
    next_map: HashMap<VertexId, VertexId>,
}

impl Geometry for Polygon {
    fn vertices(&self) -> Vec<&Vertex> {
        self.vertex_map.values().collect_vec()
    }

    fn edges(&self) -> HashSet<(VertexId, VertexId)> {
        // TODO could cache this and clear on modification
        let mut edges = HashSet::new();
        let anchor_id = self.vertices()[0].id;
        // TODO instead of unwrapping these, this function could
        // return result with an associated error type
        let mut current = self.get_vertex(&anchor_id).unwrap();
        loop {
            let next = self.get_next_vertex(&current.id).unwrap();
            edges.insert((current.id, next.id));
            current = next;
            if current.id == anchor_id {
                break;
            }
        }
        edges
    }
}

impl Polygon {
    pub fn from_coords(coords: Vec<(f64, f64)>) -> Polygon {
        let mut vertex_map = HashMap::new();
        let mut prev_map = HashMap::new();
        let mut next_map = HashMap::new();

        // TODO currently the IDs are simply generated starting
        // at 0 and incrementing. If you want to keep this route,
        // will need to track index on self so that new vertices
        // could be added. Tried using unique_id::SequenceGenerator
        // but it was global which was harder to test with
        let num_points = coords.len();
        let vertex_ids = (0..num_points).map(VertexId::from).collect::<Vec<_>>();

        for (i, coord) in coords.into_iter().enumerate() {
            let prev_id = vertex_ids[(i + num_points - 1) % num_points];
            let curr_id = vertex_ids[i];
            let next_id = vertex_ids[(i + num_points + 1) % num_points];
            let v = Vertex::new(curr_id, coord.0, coord.1);
            vertex_map.insert(curr_id, v);
            prev_map.insert(curr_id, prev_id);
            next_map.insert(curr_id, next_id);
        }

        let polygon = Polygon {
            vertex_map,
            prev_map,
            next_map,
        };
        polygon.validate();
        polygon
    }

    pub fn from_vertices(vertices: Vec<Vertex>) -> Polygon {
        let mut vertex_map = HashMap::new();
        let mut prev_map = HashMap::new();
        let mut next_map = HashMap::new();

        let num_vs = vertices.len();
        let vertex_ids = vertices.iter().map(|v| v.id).collect_vec();

        for (i, v) in vertices.iter().cloned().enumerate() {
            let prev_id = vertex_ids[(i + num_vs - 1) % num_vs];
            let next_id = vertex_ids[(i + num_vs + 1) % num_vs];
            prev_map.insert(v.id, prev_id);
            next_map.insert(v.id, next_id);
            vertex_map.insert(v.id, v);
        }

        let polygon = Polygon {
            vertex_map,
            prev_map,
            next_map,
        };
        polygon.validate();
        polygon
    }

    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Polygon, FileError> {
        let points_str: String = fs::read_to_string(path)?;
        let coords: Vec<(f64, f64)> = serde_json::from_str(&points_str)?;
        Ok(Polygon::from_coords(coords))
    }

    pub fn to_json<P: AsRef<Path>>(&self, path: P) -> Result<(), FileError> {
        let coords = self
            .sorted_vertices()
            .iter()
            .map(|v| v.coords())
            .collect_vec();
        let points_str = serde_json::to_string_pretty(&coords)?;
        fs::write(path, points_str)?;
        Ok(())
    }

    pub fn vertex_ids(&self) -> Vec<VertexId> {
        self.vertex_map.keys().cloned().collect_vec()
    }

    pub fn sorted_vertices(&self) -> Vec<&Vertex> {
        let mut vertices = self.vertices();
        vertices.sort_by_key(|v| v.id);
        vertices
    }

    pub fn min_angle_sorted_vertices(&self) -> Vec<&Vertex> {
        let v0 = self.rightmost_lowest_vertex();
        let mut v = v0.clone();
        v.x -= 1.0; // Arbitrary distance
        let e0 = LineSegment::from_vertices(v, v0.clone());

        let vertices: Vec<_> = self
            .vertices()
            .into_iter()
            .filter(|v| v.id != v0.id)
            // Break ties by sorting farthest to closest so that the dedup
            // will keep the first instance (farthest) so it will favor
            // extreme points
            .sorted_by_key(|v| (OF(e0.angle_to_vertex(&v)), Reverse(OF(v0.distance_to(v)))))
            .dedup_by(|a, b| e0.angle_to_vertex(&a) == e0.angle_to_vertex(&b))
            .collect();
        vertices
    }

    pub fn area(&self) -> f64 {
        let mut area = 0.0;
        let anchor = self.vertices()[0];
        for v1 in self.vertex_map.values() {
            let v2 = self.get_next_vertex(&v1.id).unwrap();
            let t = Triangle::from_vertices(anchor.clone(), v1.clone(), v2.clone());
            area += t.area();
        }
        area
    }

    pub fn prev_vertex(&self, id: &VertexId) -> Option<VertexId> {
        self.prev_map.get(id).cloned()
    }

    pub fn next_vertex(&self, id: &VertexId) -> Option<VertexId> {
        self.next_map.get(id).cloned()
    }

    pub fn remove_vertex(&mut self, id: &VertexId) -> Option<Vertex> {
        if let Some(v) = self.vertex_map.remove(id) {
            // TODO don't unwrap
            let v_prev = self.prev_map.remove(&v.id).unwrap();
            let v_next = self.next_map.remove(&v.id).unwrap();
            self.next_map.insert(v_prev, v_next);
            self.prev_map.insert(v_next, v_prev);
            return Some(v);
        }
        None
    }

    pub fn get_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        self.vertex_map.get(id)
    }

    pub fn get_prev_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        let prev_id = self.prev_vertex(id).unwrap(); // TODO don't unwrap
        self.vertex_map.get(&prev_id)
    }

    pub fn get_next_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        let next_id = self.next_vertex(id).unwrap(); // TODO don't unwrap
        self.vertex_map.get(&next_id)
    }

    pub fn get_vertex_mut(&mut self, id: &VertexId) -> Option<&mut Vertex> {
        self.vertex_map.get_mut(id)
    }

    pub fn get_line_segment(&self, id_1: &VertexId, id_2: &VertexId) -> Option<LineSegment> {
        if let Some(v1) = self.get_vertex(id_1) {
            if let Some(v2) = self.get_vertex(id_2) {
                return Some(LineSegment::from_vertices(v1.clone(), v2.clone()));
            }
        }
        None
    }

    pub fn get_triangle(
        &self,
        id_1: &VertexId,
        id_2: &VertexId,
        id_3: &VertexId,
    ) -> Option<Triangle> {
        if let Some(v1) = self.vertex_map.get(id_1) {
            if let Some(v2) = self.vertex_map.get(id_2) {
                if let Some(v3) = self.vertex_map.get(id_3) {
                    return Some(Triangle::from_vertices(v1.clone(), v2.clone(), v3.clone()));
                }
            }
        }
        None
    }

    pub fn get_polygon(&self, ids: impl IntoIterator<Item = VertexId>) -> Polygon {
        // Note this is currently sorting as its primary use is in convex hull,
        // if it proves useful for this sorting to be optional (i.e. assume the
        // order of input IDs is as desired) then can make this optional
        let vertices = ids
            .into_iter()
            .map(|id| self.get_vertex(&id).unwrap()) // TODO don't unwrap
            .cloned()
            .sorted_by_key(|v| v.id)
            .collect_vec();
        Polygon::from_vertices(vertices)
    }

    pub fn distance_between(&self, id_1: &VertexId, id_2: &VertexId) -> f64 {
        self.get_line_segment(id_1, id_2).unwrap().length()
    }

    fn in_cone(&self, a: &Vertex, b: &Vertex) -> bool {
        let ab = LineSegment::from_vertices(a.clone(), b.clone());
        let ba = &ab.reverse();
        // TODO instead of unwrap, return result with error
        let a0 = self.get_prev_vertex(&a.id).unwrap();
        let a1 = self.get_next_vertex(&a.id).unwrap();

        if a0.left_on(&LineSegment::from_vertices(a.clone(), a1.clone())) {
            return a0.left(&ab) && a1.left(ba);
        }

        // Otherwise a is reflexive
        !(a1.left_on(&ab) && a0.left_on(ba))
    }

    pub fn diagonal(&self, a: &Vertex, b: &Vertex) -> bool {
        self.in_cone(a, b) && self.in_cone(b, a) && self.diagonal_internal_external(a, b)
    }

    fn diagonal_internal_external(&self, a: &Vertex, b: &Vertex) -> bool {
        let ab = &LineSegment::from_vertices(a.clone(), b.clone());
        for (id1, id2) in self.edges() {
            // TODO instead of unwrap, return result with error
            let e = self.get_line_segment(&id1, &id2).unwrap();
            if !e.connected_to(ab) && e.intersects(ab) {
                return false;
            }
        }
        true
    }

    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.min_x(), self.max_x(), self.min_y(), self.max_y())
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        for v in self.vertex_map.values_mut() {
            v.translate(x, y);
        }
    }

    pub fn rotate_about_origin(&mut self, radians: f64) {
        for v in self.vertex_map.values_mut() {
            v.rotate_about_origin(radians);
        }
    }

    pub fn rotate_about_vertex(&mut self, radians: f64, vertex: &Vertex) {
        for v in self.vertex_map.values_mut() {
            v.rotate_about_vertex(radians, vertex);
        }
    }

    pub fn round_coordinates(&mut self) {
        for v in self.vertex_map.values_mut() {
            v.round_coordinates();
        }
    }

    pub fn validate(&self) {
        self.validate_num_vertices();
        self.validate_cycle();
        self.validate_edge_intersections();
    }

    fn validate_num_vertices(&self) {
        let num_vertices = self.num_vertices();
        assert!(
            num_vertices >= 3,
            "Polygon must have at least 3 vertices, \
            this one has {num_vertices}"
        );
    }

    fn validate_cycle(&self) {
        // Walk the chain and terminate once a loop closure is
        // encountered, then validate every vertex was visited
        // once. Note the loop must terminate since there are
        // finite vertices and visited vertices are tracked.
        let anchor = self.vertices()[0];
        let mut current = anchor;
        let mut visited = HashSet::<VertexId>::new();

        loop {
            visited.insert(current.id);
            // TODO don't unwrap
            current = self.get_next_vertex(&current.id).unwrap();
            if current.id == anchor.id || visited.contains(&current.id) {
                break;
            }
        }

        let mut not_visited = HashSet::<VertexId>::new();
        for v in self.sorted_vertices() {
            if !visited.contains(&v.id) {
                not_visited.insert(v.id);
            }
        }
        assert!(
            not_visited.is_empty(),
            "Expected vertex chain to form a cycle but these \
            vertices were not visited: {not_visited:?}"
        );
    }

    fn validate_edge_intersections(&self) {
        let mut edges = Vec::new();
        let anchor_id = self.vertices()[0].id;
        let mut current = self.get_vertex(&anchor_id).unwrap();
        loop {
            let next = self.get_next_vertex(&current.id).unwrap();
            let ls = LineSegment::from_vertices(current.clone(), next.clone());
            edges.push(ls);
            current = next;
            if current.id == anchor_id {
                break;
            }
        }

        for i in 0..(edges.len() - 1) {
            let e1 = &edges[i];
            // Adjacent edges should share a common vertex
            assert!(e1.incident_to(&edges[i + 1].v1));
            for e2 in edges.iter().take(edges.len() - 1).skip(i + 2) {
                // Non-adjacent edges should have no intersection
                assert!(!e1.intersects(e2));
                assert!(!e1.incident_to(&e2.v1));
                assert!(!e1.incident_to(&e2.v2));
                assert!(!e2.intersects(e1));
                assert!(!e2.incident_to(&e1.v1));
                assert!(!e2.incident_to(&e1.v2));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::*;
    use crate::F64_ASSERT_PRECISION;

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rstest::rstest;
    use rstest_reuse::{self, *};
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI, SQRT_2};
    use tempfile::NamedTempFile;

    #[test]
    #[should_panic]
    fn test_invalid_polygon_not_enough_vertices() {
        let coords = vec![(1.0, 2.0), (3.0, 4.0)];
        let _ = Polygon::from_coords(coords);
    }

    #[test]
    #[should_panic]
    fn test_invalid_polygon_not_simple() {
        let coords = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (4.0, 1.0)];
        let _ = Polygon::from_coords(coords);
    }

    #[apply(all_polygons)]
    fn test_json(case: PolygonTestCase) {
        let filename = NamedTempFile::new().unwrap().into_temp_path();
        let _ = case.polygon.to_json(&filename);
        let new_polygon = Polygon::from_json(&filename).unwrap();
        assert_eq!(case.polygon, new_polygon);
    }

    #[test]
    // TODO could expand this test to polygon cases
    fn test_min_max() {
        let coords = vec![
            (0.0, 0.0),
            (5.0, -1.0),
            (7.0, 6.0),
            (-4.0, 8.0),
            (-2.0, -3.0),
        ];
        let polygon = Polygon::from_coords(coords);
        assert_eq!(polygon.min_x(), -4.0);
        assert_eq!(polygon.max_x(), 7.0);
        assert_eq!(polygon.min_y(), -3.0);
        assert_eq!(polygon.max_y(), 8.0);
    }

    #[test]
    // TODO could expand this test to polygon cases
    fn test_lowest_vertex() {
        let coords = vec![
            (0.0, 0.0),
            (5.0, -1.0),
            (7.0, 6.0),
            (-4.0, 8.0),
            (-2.0, -3.0),
        ];
        let polygon = Polygon::from_coords(coords);
        let lowest = polygon.leftmost_lowest_vertex();
        assert_eq!(lowest.x, -2.0);
        assert_eq!(lowest.y, -3.0);
    }

    #[apply(all_polygons)]
    fn test_area(case: PolygonTestCase) {
        let area = case.polygon.area();
        assert_eq!(area, case.metadata.area);
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
    fn test_rotation_about_origin(
        case: PolygonTestCase,
        #[values(PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8)] radians: f64,
    ) {
        let mut polygon = case.polygon;
        polygon.rotate_about_origin(radians);
        polygon.validate();
        assert_eq!(polygon.num_edges(), case.metadata.num_edges);
        assert_eq!(polygon.num_vertices(), case.metadata.num_vertices);
        assert_approx_eq!(polygon.area(), case.metadata.area, F64_ASSERT_PRECISION);
    }

    #[apply(all_polygons)]
    fn test_rotation_about_vertex(
        case: PolygonTestCase,
        #[values(PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8)] radians: f64,
        #[values((5.2, 10.0), (-43.0, PI), (SQRT_2, 1e8))] coord: (f64, f64),
    ) {
        let mut polygon = case.polygon;
        let v = Vertex::new(VertexId::default(), coord.0, coord.1);
        polygon.rotate_about_vertex(radians, &v);
        assert_eq!(polygon.num_edges(), case.metadata.num_edges);
        assert_eq!(polygon.num_vertices(), case.metadata.num_vertices);
        assert_approx_eq!(polygon.area(), case.metadata.area, F64_ASSERT_PRECISION);
    }

    #[apply(all_polygons)]
    fn test_attributes(case: PolygonTestCase) {
        assert_eq!(case.polygon.num_edges(), case.metadata.num_edges);
        assert_eq!(case.polygon.num_vertices(), case.metadata.num_vertices);
        // This meta-assert is only valid for polygons without holes, holes
        // are not yet supported. Will need a flag in the metadata to know
        // if holes are present and then this assert would be conditional
        assert_eq!(case.metadata.num_edges, case.metadata.num_vertices);
    }
}
