use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::convex_hull::{ConvexHull, Edge};
use crate::{
    bounding_box::BoundingBox, 
    error::FileError,
    line_segment::LineSegment, 
    point::Point, 
    triangle::Triangle, 
    triangulation::{EarNotFoundError, TriangleVertexIds, Triangulation}, 
    vertex::{Vertex, VertexId},
};


#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    vertex_map: HashMap<VertexId, Vertex>,
}


impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        let mut vertex_map = HashMap::new();

        // TODO currently the IDs are simply generated starting
        // at 0 and incrementing. If you want to keep this route,
        // will need to track index on self so that new vertices
        // could be added. Tried using unique_id::SequenceGenerator
        // but it was global which was harder to test with
        let num_points = points.len();
        let vertex_ids = (0..num_points)
            .map(VertexId::from)
            .collect::<Vec<_>>();

        for (i, point) in points.into_iter().enumerate() {
            let prev_id = vertex_ids[(i + num_points - 1) % num_points];
            let curr_id = vertex_ids[i];
            let next_id = vertex_ids[(i + num_points + 1) % num_points];
            let v = Vertex::new(point, curr_id, prev_id, next_id);
            vertex_map.insert(curr_id, v);
        }



        let polygon = Polygon { vertex_map };
        polygon.validate();
        polygon
    }

    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Polygon, FileError> {
        let points_str: String = fs::read_to_string(path)?;
        let points: Vec<Point> = serde_json::from_str(&points_str)?;
        Ok(Polygon::new(points))
    }

    pub fn to_json<P: AsRef<Path>>(&self, path: P) -> Result<(), FileError>{
        let points = self.sorted_points();
        let points_str = serde_json::to_string_pretty(&points)?;
        fs::write(path, points_str)?;
        Ok(())
    }

    pub fn anchor(&self) -> &Vertex {
        // TODO I'm not yet convinced this is something I want, ultimately
        // need something to initiate algorithms in the vertex chain.
        // Could consider only exposing keys and then having polygon gen
        // an anchor
        self.vertex_map.values().collect::<Vec<_>>()[0]
    }

    pub fn num_edges(&self) -> usize {
        self.edges().len()
    }

    pub fn num_vertices(&self) -> usize {
        self.vertex_map.len()
    }

    pub fn sorted_ids(&self) -> Vec<&VertexId> {
        let mut ids: Vec<_> = self.vertex_map.keys().collect();
        ids.sort();
        ids
    }

    pub fn sorted_points(&self) -> Vec<Point> {
        self.sorted_vertices()
            .iter()
            .map(|v| v.coords.clone())
            .collect::<Vec<Point>>()
    }

    pub fn sorted_vertices(&self) -> Vec<&Vertex> {
        let mut vertices = self.vertex_map.values().collect_vec();
        vertices.sort_by(|a, b| a.id.cmp(&b.id));
        vertices
    }

    pub fn area(&self) -> f64 {
        let mut area = 0.0;
        let anchor = self.anchor();
        for v1 in self.vertex_map.values() {
            let v2 = self.get_vertex(&v1.next).unwrap(); 
            area += Triangle::from_vertices(anchor, v1, v2).area();
        }
        area
    }

    pub fn area_from_triangulation(&self, triangulation: &Triangulation) -> f64 {
        // Computes area from a triangulation as the sum of the area of 
        // the individual triangles that constitute the triangulation.
        // This value should always be exactly equal to `self.area()`.
        let mut area = 0.0;
        for (p1, p2, p3) in triangulation.to_points().iter() {
            area += Triangle::new(p1, p2, p3).area();
        }
        area
    }

    pub fn triangulation(&self) -> Triangulation {
        let mut triangulation = Triangulation::new(self);
        let mut polygon = self.clone();

        while polygon.num_vertices() > 3 {
            let id = polygon.find_ear()
                .expect("valid polygons with 3 or more vertices should have an ear");
            // TODO instead of unwrap, return result with error
            let v = polygon.remove_vertex(&id).unwrap();
            triangulation.push(TriangleVertexIds(v.prev, id, v.next));
        }
        // At this stage there should be exactly 3 vertices left,
        // which form the final triangle of the triangulation
        let v = polygon.anchor();
        triangulation.push(TriangleVertexIds(v.prev, v.id, v.next));

        triangulation
    }

    fn find_ear(&self) -> Result<VertexId, EarNotFoundError> {
        for v in self.vertex_map.values() {
            let prev = self.get_vertex(&v.prev).unwrap();
            let next = self.get_vertex(&v.next).unwrap();
            if self.diagonal(prev, next) {
                return Ok(v.id);
            }
        }
        Err(EarNotFoundError)
    }

    fn remove_vertex(&mut self, id: &VertexId) -> Option<Vertex> {
        if let Some(v) = self.vertex_map.remove(id) {
            // TODO this would be an error condition if there was
            // a vertex for which prev/next weren't in the map,
            // instead of unwrap could have this func return 
            // result with an error tailored to this case
            self.get_vertex_mut(&v.prev).unwrap().next = v.next;
            self.get_vertex_mut(&v.next).unwrap().prev = v.prev;
            return Some(v);
        }
        None
    }

    pub fn get_vertex(&self, id: &VertexId) -> Option<&Vertex> {
        self.vertex_map.get(id)
    }

    pub fn get_vertex_mut(&mut self, id: &VertexId) -> Option<&mut Vertex> {
        self.vertex_map.get_mut(id)
    }

    pub fn get_point(&self, id: &VertexId) -> Option<Point> {
        if let Some(v) = self.get_vertex(id) {
            return Some(v.coords.clone())
        }
        None
    }

    pub fn get_line_segment(&self, id_1: &VertexId, id_2: &VertexId) -> Option<LineSegment> {
        if let Some(v1) = self.get_vertex(id_1) {
            if let Some(v2) = self.get_vertex(id_2) {
                return Some(LineSegment::from_vertices(v1, v2))
            }
        }
        None
    }

    pub fn get_triangle(&self, id_1: &VertexId, id_2: &VertexId, id_3: &VertexId) -> Option<Triangle> {
        if let Some(v1) = self.vertex_map.get(id_1) {
            if let Some(v2) = self.vertex_map.get(id_2) {
                if let Some(v3) = self.vertex_map.get(id_3) {
                    return Some(Triangle::from_vertices(v1, v2, v3));
                }
            }
        }
        None
    }

    pub fn edges(&self) -> HashSet<(VertexId, VertexId)> {
        // TODO could cache this and clear on modification
        let mut edges = HashSet::new();
        let anchor_id = self.anchor().id;
        // TODO instead of unwrapping these, this function could
        // return result with an associated error type
        let mut current = self.get_vertex(&anchor_id).unwrap();
        loop {
            edges.insert((current.id, current.next));
            current = self.get_vertex(&current.next).unwrap();
            if current.id == anchor_id {
                break;
            }
        }
        edges
    }
    
    fn in_cone(&self, a: &Vertex, b: &Vertex) -> bool {
        let ab = LineSegment::from_vertices(a, b);
        let ba = &ab.reverse();
        // TODO instead of unwrap, return result with error
        let a0 = self.get_vertex(&a.prev).unwrap();
        let a1 = self.get_vertex(&a.next).unwrap();

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
            // TODO instead of unwrap, return result with error
            let e = self.get_line_segment(&id1, &id2).unwrap();
            if !e.connected_to(ab) && e.intersects(ab) {
                return false;
            }
        } 
        true
    }

    pub fn interior_points(&self) -> HashSet<VertexId> {
        // This is just an alias to the current best implementation
        self.interior_points_from_extreme_edges()
    }

    pub fn interior_points_from_extreme_edges(&self) -> HashSet<VertexId> {
        let ids: HashSet<_> = self.vertex_map.keys().cloned().collect();
        let extreme_ids = self.extreme_points_from_extreme_edges();
        &ids - &extreme_ids
    }

    pub fn interior_points_from_triangle_checks(&self) -> HashSet<VertexId> {
        let mut interior_points = HashSet::new();
        let ids: HashSet<_> = self.vertex_map.keys().cloned().collect();

        // Don't be fooled by the runtime here, it's iterating over all
        // permutations, which is n! / (n-4)! = n * (n-1) * (n-2) * (n-3), 
        // so it's still O(n^4), this is just more compact than 4 nested
        // for-loops.
        for perm in ids.into_iter().permutations(4) {
            // TODO instead of unwrap, return result with error
            let p = self.get_point(&perm[0]).unwrap();
            let triangle = self.get_triangle(&perm[1], &perm[2], &perm[3]).unwrap();
            if triangle.contains(p) {
                interior_points.insert(perm[0]);
            }
        }
        interior_points
    }

    pub fn extreme_edges(&self) -> Vec<(VertexId, VertexId)> {
        // NOTE: This is O(n^3)
        let mut extreme_edges = Vec::new();
        let ids = self.vertex_map.keys().cloned().collect_vec();

        for id1 in ids.iter() {
            for id2 in ids.iter() {
                if id2 == id1 {
                    continue;
                }
                // TODO instead of unwrap, return result with error
                let ls = self.get_line_segment(id1, id2).unwrap();
                let mut is_extreme = true;
                for id3 in ids.iter() {
                    if id3 == id1 || id3 == id2 {
                        continue;
                    }
                    // TODO instead of unwrap, return result with error
                    let p = self.get_point(id3).unwrap();
                    if !p.left_on(&ls) {
                        is_extreme = false;
                        break;
                    }
                }
                if is_extreme {
                    extreme_edges.push((*id1, *id2));
                }
            }
        }
        extreme_edges
    }

    pub fn extreme_points(&self) -> HashSet<VertexId> {
        // This one is an alias to the current best implementation
        self.extreme_points_from_extreme_edges()
    }

    pub fn extreme_points_from_extreme_edges(&self) -> HashSet<VertexId> {
        // NOTE: This is O(n^3) since the extreme edges computation
        // has that runtime
        let mut extreme_points = HashSet::new();
        for (id1, id2) in self.extreme_edges().into_iter() {
            extreme_points.insert(id1);
            extreme_points.insert(id2);
        }
        extreme_points
    }

    pub fn extreme_points_from_interior_points(&self) -> HashSet<VertexId> {
        // NOTE: This is slow O(n^4) since the interior point 
        // computation being used has that runtime.
        let ids: HashSet<_> = self.vertex_map.keys().cloned().collect();
        let interior_ids = self.interior_points_from_triangle_checks();
        &ids - &interior_ids
    }

    pub fn convex_hull(&self) -> ConvexHull {
        // Alias for the best-performing hull algorithm implemented
        self.convex_hull_from_gift_wrapping()
    }

    // TODO will need to think about return types, this algorithm will
    // only return I believe a set of extreme points. So maybe it makes
    // sense to define the ConvexHull with that representation, and then
    // offer an ability to get the edges from it? The set of extreme
    // points would be more minimal
    pub fn convex_hull_from_quick_hull(&self) -> HashSet<VertexId> {
        let mut convex_hull = HashSet::new();
        let mut stack = Vec::new();

        let x = self.lowest_rightmost_vertex().id;
        let y = self.highest_leftmost_vertex().id;
        let xy = self.get_line_segment(&x, &y).unwrap();
        let s = self.vertex_map
            .values()
            .filter(|v| v.id != x && v.id != y)
            .collect_vec();

        convex_hull.insert(x);
        convex_hull.insert(y);

        let (s1, s2): (Vec<_>, Vec<_>) = s
            .into_iter()
            .partition(|v| v.right(&xy));
        
        if !s1.is_empty() { stack.push((x, y, s1)) };
        if !s2.is_empty() { stack.push((y, x, s2)) };

        loop {
            let (a, b, s) = stack.pop().unwrap();
            println!("POINTS: {:?}", s);
            println!("A: {:?}, B: {:?}", a, b);
            let ab = self.get_line_segment(&a, &b).unwrap();

            let c = s
                .iter()
                .max_by_key(|v| OrderedFloat(ab.distance_to_point(&v.coords)))
                .unwrap()
                .id;
            println!("C: {:?}", c);
            convex_hull.insert(c);

            let ac = self.get_line_segment(&a, &c).unwrap();
            let cb = self.get_line_segment(&c, &b).unwrap();

            let s1 = s
                .iter()
                .copied() // TODO remove this, couldn't figure out refs
                .filter(|v| v.right(&ac))
                .collect_vec();

            let s2 = s
                .iter()
                .copied() // TODO remove this, couldn't figure out refs
                .filter(|v| v.right(&cb))
                .collect_vec();

            if !s1.is_empty() { println!("PUSHING: a={:?}, c={:?}, s1={:?}", a, c, s1); stack.push((a, c, s1)); }
            if !s2.is_empty() { println!("PUSHING: c={:?}, b={:?}, s2={:?}", c, b, s2); stack.push((c, b, s2)); }
            if stack.is_empty() { break; }
        }
        convex_hull
    }

    pub fn convex_hull_from_gift_wrapping(&self) -> ConvexHull {
        let mut hull = ConvexHull::default();
        // Form a horizontal line terminating at lowest point to start
        let v0 = self.leftmost_lowest_vertex();
        let mut p = v0.coords.clone();
        p.x -= 1.0;  // Arbitrary distance
        let mut current_edge = LineSegment::new(&p, &v0.coords);
        let mut current_vertex_id = v0.id;

        // Perform gift-wrapping, using the previous hull edge as a vector to 
        // find the point with the least CCW angle w.r.t. the vector. Connect 
        // that point to the current terminal vertex to form the newest hull 
        // edge. Repeat until we reach the starting vertex again.
        loop {
            let min_angle_vertex_id = self.vertex_map
                .values()
                .filter(|v| v.id != current_vertex_id)
                .min_by_key(|v| OrderedFloat(current_edge.angle_to_point(&v.coords)))
                .unwrap()
                .id;

            hull.edges.push(Edge(current_vertex_id, min_angle_vertex_id));

            current_edge = self.get_line_segment(
                &current_vertex_id, &min_angle_vertex_id
            ).unwrap();
            current_vertex_id = min_angle_vertex_id;
            if current_vertex_id == v0.id {
                break;
            }
        }
        hull
    }

    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.min_x(), self.max_x(), self.min_y(), self.max_y())
    }

    pub fn min_x(&self) -> f64 {
        self.vertex_map.values().fold(f64::MAX, |acc, v| acc.min(v.coords.x))
    }

    pub fn max_x(&self) -> f64 {
        self.vertex_map.values().fold(f64::MIN, |acc, v| acc.max(v.coords.x))
    }

    pub fn min_y(&self) -> f64 {
        self.vertex_map.values().fold(f64::MAX, |acc, v| acc.min(v.coords.y))
    }

    pub fn max_y(&self) -> f64 {
        self.vertex_map.values().fold(f64::MIN, |acc, v| acc.max(v.coords.y))
    }

    pub fn leftmost_lowest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertex_map.values().collect_vec();
        vertices.sort_by_key(|v| (OrderedFloat(v.coords.y), OrderedFloat(v.coords.x)));
        vertices[0]
    }

    pub fn rightmost_lowest_vertex(&self) -> &Vertex {
        let mut vertices = self.vertex_map.values().collect_vec();
        vertices.sort_by_key(|v| (OrderedFloat(v.coords.y), OrderedFloat(-v.coords.x)));
        vertices[0]
    }

    pub fn lowest_rightmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertex_map.values().collect_vec();
        vertices.sort_by_key(|v| (OrderedFloat(-v.coords.x), OrderedFloat(v.coords.y)));
        vertices[0]
    }

    pub fn highest_leftmost_vertex(&self) -> &Vertex {
        let mut vertices = self.vertex_map.values().collect_vec();
        vertices.sort_by_key(|v| (OrderedFloat(v.coords.x), OrderedFloat(-v.coords.y)));
        vertices[0]
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

    pub fn rotate_about_point(&mut self, radians: f64, point: &Point) {
        for v in self.vertex_map.values_mut() {
            v.rotate_about_point(radians, point);
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
        let anchor = self.anchor();
        let mut current = self.anchor();
        let mut visited = HashSet::<VertexId>::new();

        loop {
            visited.insert(current.id);
            // TODO don't unwrap
            current = self.vertex_map.get(&current.next).unwrap();
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
        let anchor_id = self.anchor().id;
        let mut current = self.get_vertex(&anchor_id).unwrap();
        loop {
            let next = self.get_vertex(&current.next).unwrap();
            let ls = LineSegment::from_vertices(current, next);
            edges.push(ls);
            current = next;
            if current.id == anchor_id {
                break;
            }
        }
        
        for i in 0..(edges.len() - 1) {
            let e1 = &edges[i];
            // Adjacent edges should share a common vertex
            assert!(e1.incident_to(edges[i+1].p1));
            for e2 in edges.iter().take(edges.len() -1).skip(i+2) {
                // Non-adjacent edges should have no intersection
                assert!(!e1.intersects(e2));
                assert!(!e1.incident_to(e2.p1));
                assert!(!e1.incident_to(e2.p2));
                assert!(!e2.intersects(e1));
                assert!(!e2.incident_to(e1.p1));
                assert!(!e2.incident_to(e1.p2));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::F64_ASSERT_PRECISION;
    use crate::util::load_polygon;

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rstest::{fixture, rstest};
    use rstest_reuse::{self, *};
    use serde::Deserialize;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI, SQRT_2};
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[derive(Deserialize)]
    struct PolygonMetadata {
        area: f64,
        convex_hull: ConvexHull,
        extreme_points: HashSet<VertexId>,
        interior_points: HashSet<VertexId>,
        num_edges: usize,
        num_triangles: usize,
        num_vertices: usize,
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

    fn load_metadata(name: &str, folder: &str) -> Result<PolygonMetadata, FileError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("polygons");
        path.push(folder);
        path.push(format!("{}.meta.json", name));
        let metadata_str: String = fs::read_to_string(path)?;
        let metadata = serde_json::from_str(&metadata_str)?;
        Ok(metadata)
    }

    #[macro_export]
    macro_rules! polygon_fixture {
        ($name:ident, $folder:expr) => {
            #[fixture]
            fn $name() -> PolygonTestCase {
                PolygonTestCase::new(
                    load_polygon(stringify!($name), stringify!($folder)).unwrap(),
                    load_metadata(stringify!($name), stringify!($folder)).unwrap(),
                )
            }
        };
    }

    polygon_fixture!(polygon_1, custom);
    polygon_fixture!(polygon_2, custom);
    polygon_fixture!(right_triangle, custom);
    polygon_fixture!(square_4x4, custom);

    polygon_fixture!(eberly_10, interesting_polygon_archive);
    polygon_fixture!(eberly_14, interesting_polygon_archive);
    polygon_fixture!(elgindy_1, interesting_polygon_archive);
    polygon_fixture!(gray_embroidery, interesting_polygon_archive);
    polygon_fixture!(held_1, interesting_polygon_archive);
    polygon_fixture!(held_3, interesting_polygon_archive);
    polygon_fixture!(held_12, interesting_polygon_archive);
    polygon_fixture!(held_7a, interesting_polygon_archive);
    polygon_fixture!(held_7b, interesting_polygon_archive);
    polygon_fixture!(held_7c, interesting_polygon_archive);
    polygon_fixture!(held_7d, interesting_polygon_archive);
    polygon_fixture!(mapbox_building, interesting_polygon_archive);
    polygon_fixture!(mapbox_dude, interesting_polygon_archive);
    polygon_fixture!(matisse_alga, interesting_polygon_archive);
    polygon_fixture!(matisse_blue, interesting_polygon_archive);
    polygon_fixture!(matisse_icarus, interesting_polygon_archive);
    polygon_fixture!(matisse_nuit, interesting_polygon_archive);
    polygon_fixture!(mei_2, interesting_polygon_archive);
    polygon_fixture!(mei_3, interesting_polygon_archive);
    polygon_fixture!(mei_4, interesting_polygon_archive);
    polygon_fixture!(mei_5, interesting_polygon_archive);
    polygon_fixture!(mei_6, interesting_polygon_archive);
    polygon_fixture!(meisters_3, interesting_polygon_archive);
    polygon_fixture!(misc_discobolus, interesting_polygon_archive);
    polygon_fixture!(misc_fu, interesting_polygon_archive);
    polygon_fixture!(seidel_3, interesting_polygon_archive);
    polygon_fixture!(skimage_horse, interesting_polygon_archive);
    polygon_fixture!(toussaint_1a, interesting_polygon_archive);

    #[template]
    #[rstest]
    #[case::right_triangle(right_triangle())]
    #[case::square_4x4(square_4x4())]
    #[case::polygon_1(polygon_1())]
    #[case::polygon_2(polygon_2())]
    #[case::eberly_10(eberly_10())]
    #[case::eberly_14(eberly_14())]
    #[case::elgindy_1(elgindy_1())]
    #[case::gray_embroidery(gray_embroidery())]
    #[case::held_1(held_1())]
    #[case::held_3(held_3())]
    #[case::held_12(held_12())]
    #[case::held_7a(held_7a())]
    #[case::held_7b(held_7b())]
    #[case::held_7c(held_7c())]
    #[case::held_7d(held_7d())]
    #[case::mapbox_building(mapbox_building())]
    #[case::mapbox_dude(mapbox_dude())]
    #[case::matisse_alga(matisse_alga())]
    #[case::matisse_blue(matisse_blue())]
    #[case::matisse_icarus(matisse_icarus())]
    #[case::matisse_nuit(matisse_nuit())]
    #[case::mei_2(mei_2())]
    #[case::mei_3(mei_3())]
    #[case::mei_4(mei_4())]
    #[case::mei_5(mei_5())]
    #[case::mei_6(mei_6())]
    #[case::meisters_3(meisters_3())]
    #[case::misc_discobolus(misc_discobolus())]
    #[case::misc_fu(misc_fu())]
    #[case::seidel_3(seidel_3())]
    #[case::skimage_horse(skimage_horse())]
    #[case::toussaint_1a(toussaint_1a())]
    fn all_polygons(#[case] case: PolygonTestCase) {}

    #[template]
    #[rstest]
    #[case::polygon_1(polygon_1())]
    #[case::polygon_2(polygon_2())]
    #[case::right_triangle(right_triangle())]
    #[case::square_4x4(square_4x4())]
    fn all_custom_polygons(#[case] case: PolygonTestCase) {}

    #[template]
    #[apply(all_custom_polygons)]
    #[case::eberly_10(eberly_10())]
    #[case::eberly_14(eberly_14())]
    #[case::elgindy_1(elgindy_1())]
    #[case::gray_embroidery(gray_embroidery())]
    #[case::held_1(held_1())]
    #[case::held_12(held_12())]
    #[case::held_3(held_3())]
    // TODO can add more test cases here, but I've just been going through
    // and manually verifying they're correct. Some have so many vertices
    // though it's not practical
    fn extreme_point_cases(#[case] case: PolygonTestCase) {}


    #[test]
    #[should_panic]
    fn test_invalid_polygon_not_enough_vertices() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let points = vec![p1, p2];
        let polygon = Polygon::new(points);
        assert_eq!(2, polygon.num_vertices());
    }

    #[test]
    #[should_panic]
    fn test_invalid_polygon_not_simple() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(2.0, 0.0);
        let p3 = Point::new(2.0, 2.0);
        let p4 = Point::new(0.0, 2.0);
        let p5 = Point::new(4.0, 1.0); // This one should break it
        let points = vec![p1, p2, p3, p4, p5];
        let polygon = Polygon::new(points);
        assert_eq!(3, polygon.num_vertices())
    }

    #[apply(all_polygons)]
    fn test_json(case: PolygonTestCase) {
        let filename = NamedTempFile::new()
            .unwrap()
            .into_temp_path();
        let _ = case.polygon.to_json(&filename);
        let new_polygon = Polygon::from_json(&filename).unwrap();
        assert_eq!(case.polygon, new_polygon);
    }

    #[test]
    // TODO could expand this test to polygon cases
    fn test_min_max() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(5.0, -1.0);
        let p3 = Point::new(7.0, 6.0);
        let p4 = Point::new(-4.0, 8.0);
        let p5 = Point::new(-2.0, -3.0);
        let points = vec![p1, p2, p3, p4, p5];
        let polygon = Polygon::new(points);
        assert_eq!(polygon.min_x(), -4.0);
        assert_eq!(polygon.max_x(), 7.0);
        assert_eq!(polygon.min_y(), -3.0);
        assert_eq!(polygon.max_y(), 8.0);
    }

    #[test]
    // TODO could expand this test to polygon cases
    fn test_lowest_vertex() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(5.0, -1.0);
        let p3 = Point::new(7.0, 6.0);
        let p4 = Point::new(-4.0, 8.0);
        let p5 = Point::new(-2.0, -3.0);
        let points = vec![p1, p2, p3, p4, p5];
        let polygon = Polygon::new(points);
        let lowest = polygon.leftmost_lowest_vertex();
        assert_eq!(lowest.coords.x, -2.0);
        assert_eq!(lowest.coords.y, -3.0);
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
        #[values(PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8)] radians: f64
    ) {
        let mut polygon = case.polygon;
        polygon.rotate_about_origin(radians);
        polygon.validate();
        assert_eq!(polygon.num_edges(), case.metadata.num_edges);
        assert_eq!(polygon.num_vertices(), case.metadata.num_vertices);
        assert_approx_eq!(polygon.area(), case.metadata.area, F64_ASSERT_PRECISION);
    }

    #[apply(all_polygons)]
    fn test_rotation_about_point(
        case: PolygonTestCase,
        #[values(PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8)] radians: f64,
        #[values(Point::new(5.2, 10.0), Point::new(-43.0, PI), Point::new(SQRT_2, 1e8))] point: Point
    ) {
        let mut polygon = case.polygon;
        polygon.rotate_about_point(radians, &point);
        assert_eq!(polygon.num_edges(), case.metadata.num_edges);
        assert_eq!(polygon.num_vertices(), case.metadata.num_vertices);
        assert_approx_eq!(polygon.area(), case.metadata.area, F64_ASSERT_PRECISION);
    }

    #[apply(all_polygons)]
    fn test_triangulation(case: PolygonTestCase) {
        let triangulation = case.polygon.triangulation();
        assert_eq!(triangulation.len(), case.metadata.num_triangles);
        // This meta-assert is only valid for polygons without holes, holes 
        // are not yet supported. Will need a flag in the metadata to know 
        // if holes are present and then this assert would be conditional
        assert_eq!(case.metadata.num_triangles, case.metadata.num_edges - 2);

        let triangulation_area = case.polygon.area_from_triangulation(&triangulation);
        assert_eq!(triangulation_area, case.metadata.area);
    }

    #[apply(extreme_point_cases)]
    fn test_interior_points_from_triangle_checks(#[case] case: PolygonTestCase) {
        let interior_points = case.polygon.interior_points_from_triangle_checks();
        assert_eq!(
            interior_points, 
            case.metadata.interior_points,
            "Extra computed: {:?}, Extra in metadata: {:?}", 
            interior_points.difference(&case.metadata.interior_points),
            case.metadata.interior_points.difference(&interior_points)
        );
    }

    #[apply(extreme_point_cases)]
    fn test_interior_points_from_extreme_edges(#[case] case: PolygonTestCase) {
        let interior_points = case.polygon.interior_points_from_extreme_edges();
        assert_eq!(
            interior_points, 
            case.metadata.interior_points,
            "Extra computed: {:?}, Extra in metadata: {:?}", 
            interior_points.difference(&case.metadata.interior_points),
            case.metadata.interior_points.difference(&interior_points)
        );
    }

    #[apply(extreme_point_cases)]
    fn test_extreme_points_from_interior_points(#[case] case: PolygonTestCase) {
        let extreme_points = case.polygon.extreme_points_from_interior_points();
        assert_eq!(
            extreme_points,
            case.metadata.extreme_points,
            "Extra computed: {:?}, Extra in metadata: {:?}", 
            extreme_points.difference(&case.metadata.extreme_points),
            case.metadata.extreme_points.difference(&extreme_points)
        );
    }

    #[apply(extreme_point_cases)]
    fn test_extreme_points_from_extreme_edges(#[case] case: PolygonTestCase) {
        let extreme_points = case.polygon.extreme_points_from_extreme_edges();
        assert_eq!(
            extreme_points,
            case.metadata.extreme_points,
            "Extra computed: {:?}, Extra in metadata: {:?}", 
            extreme_points.difference(&case.metadata.extreme_points),
            case.metadata.extreme_points.difference(&extreme_points)
        );
    }

    // TODO will want to parametrize on more polygons when defined
    #[apply(all_custom_polygons)]
    fn test_convex_hull_from_gift_wrapping(#[case] case: PolygonTestCase) {
        let convex_hull = case.polygon.convex_hull_from_gift_wrapping();
        assert_eq!(convex_hull, case.metadata.convex_hull);
    }

    // TODO will want to parametrize on more polygons when defined
    #[apply(extreme_point_cases)]
    fn test_convex_hull_from_quick_hull(#[case] case: PolygonTestCase) {
        let convex_hull = case.polygon.convex_hull_from_quick_hull();
        // TODO need to sort out types
        assert_eq!(convex_hull, case.metadata.extreme_points);
    }

    #[apply(all_polygons)]
    fn test_attributes(case: PolygonTestCase) {
        assert_eq!(case.polygon.num_edges(), case.metadata.num_edges);
        assert_eq!(case.polygon.num_vertices(), case.metadata.num_vertices);

        // Not all test cases have these defined so only assert on ones that do
        let num_extreme_points = case.metadata.extreme_points.len();
        let num_interior_points = case.metadata.interior_points.len();
        if num_extreme_points > 0 || num_interior_points > 0 {
            assert_eq!(
                num_extreme_points + num_interior_points, 
                case.metadata.num_vertices,
            );
            assert!(
                case.metadata.extreme_points.is_disjoint(&case.metadata.interior_points)
            );
        }
        // This meta-assert is only valid for polygons without holes, holes 
        // are not yet supported. Will need a flag in the metadata to know 
        // if holes are present and then this assert would be conditional
        assert_eq!(case.metadata.num_edges, case.metadata.num_vertices);
    }
}