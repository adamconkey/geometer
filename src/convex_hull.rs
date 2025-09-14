use itertools::Itertools;
use log::{debug, info, trace};
use ordered_float::OrderedFloat as OF;
use std::collections::HashSet;
use std::fmt;

use crate::{geometry::Geometry, polygon::Polygon, vertex::VertexId};

#[derive(Default)]
pub struct ConvexHullTracerStep {
    pub hull: Vec<VertexId>,
    pub next_vertex: Option<VertexId>,
    pub upper_tangent_vertex: Option<VertexId>,
    pub lower_tangent_vertex: Option<VertexId>,
}

impl fmt::Display for ConvexHullTracerStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\tHull Vertices: {:?}", self.hull)?;
        if let Some(n_v) = self.next_vertex {
            writeln!(f, "\tNext Vertex: {:?}", n_v)?;
        }
        if let Some(ut_v) = self.upper_tangent_vertex {
            writeln!(f, "\tUpper Tangent Vertex: {:?}", ut_v)?;
        }
        if let Some(lt_v) = self.lower_tangent_vertex {
            writeln!(f, "\tLower Tangent Vertex: {:?}", lt_v)?;
        }
        Ok(())
    }
}

impl ConvexHullTracerStep {
    pub fn hull_tail(&self, num_elements: usize) -> Vec<VertexId> {
        self.hull[self.hull.len() - num_elements..].to_vec()
    }
}

#[derive(Default)]
pub struct ConvexHullTracer {
    pub steps: Vec<ConvexHullTracerStep>,
}

impl fmt::Debug for ConvexHullTracer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, step) in self.steps.iter().enumerate() {
            write!(f, "STEP {}:\n{}", i, step)?;
        }
        Ok(())
    }
}

pub trait ConvexHullComputer {
    fn convex_hull(&self, polygon: &Polygon, tracer: &mut Option<ConvexHullTracer>) -> Polygon;
}

#[derive(Default)]
pub struct InteriorPoints;

impl InteriorPoints {
    pub fn interior_points(&self, polygon: &Polygon) -> HashSet<VertexId> {
        let mut interior_points = HashSet::new();
        let ids = polygon.vertex_ids();

        // Don't be fooled by the runtime here, it's iterating over all
        // permutations, which is n! / (n-4)! = n * (n-1) * (n-2) * (n-3),
        // so it's still O(n^4), just more compact than 4 nested for-loops.
        for perm in ids.into_iter().permutations(4) {
            let v = polygon.get_vertex(&perm[0]).unwrap();
            let triangle = polygon.get_triangle(&perm[1], &perm[2], &perm[3]).unwrap();
            if triangle.contains(v) {
                interior_points.insert(perm[0]);
            }
        }
        interior_points
    }
}

impl ConvexHullComputer for InteriorPoints {
    fn convex_hull(&self, polygon: &Polygon, _tracer: &mut Option<ConvexHullTracer>) -> Polygon {
        let interior_ids = self.interior_points(polygon);
        let all_ids = HashSet::from_iter(polygon.vertex_ids());
        let hull_ids = &all_ids - &interior_ids;
        polygon.get_polygon(hull_ids, true, false)
    }
}

#[derive(Default)]
pub struct ExtremeEdges;

impl ExtremeEdges {
    pub fn extreme_edges(&self, polygon: &Polygon) -> Vec<(VertexId, VertexId)> {
        // NOTE: This is O(n^3)
        let mut extreme_edges = Vec::new();
        let ids = polygon.vertex_ids();

        for perm in ids.iter().permutations(2) {
            let ls = polygon.get_line_segment(perm[0], perm[1]).unwrap();
            let mut is_extreme = true;
            for id3 in ids.iter().filter(|id| !perm.contains(id)) {
                let v = polygon.get_vertex(id3).unwrap();
                if !v.left_on(&ls) {
                    is_extreme = false;
                    break;
                }
            }
            if is_extreme {
                extreme_edges.push((*perm[0], *perm[1]));
            }
        }

        extreme_edges
    }
}

impl ConvexHullComputer for ExtremeEdges {
    fn convex_hull(&self, polygon: &Polygon, _tracer: &mut Option<ConvexHullTracer>) -> Polygon {
        let hull_ids = self
            .extreme_edges(polygon)
            .into_iter()
            .map(|(id1, _)| id1)
            .sorted()
            .dedup();
        polygon.get_polygon(hull_ids, false, true)
    }
}

#[derive(Default)]
pub struct GiftWrapping;

impl ConvexHullComputer for GiftWrapping {
    fn convex_hull(&self, polygon: &Polygon, _tracer: &mut Option<ConvexHullTracer>) -> Polygon {
        let mut hull_ids = HashSet::new();
        let v0 = polygon.rightmost_lowest_vertex();
        hull_ids.insert(v0.id);
        // Perform gift-wrapping, using the previous hull edge as a vector to
        // find the point with the least CCW angle w.r.t. the vector. Connect
        // that point to the current terminal vertex to form the newest hull
        // edge. Repeat until we reach the starting vertex again.
        let mut v = v0;
        let mut e = None;
        while v.id != v0.id || e.is_none() {
            let v_min_angle = polygon.min_angle_sorted_vertices(Some(v), e)[0];
            e = polygon.get_line_segment(&v.id, &v_min_angle.id);
            v = v_min_angle;
            hull_ids.insert(v.id);
        }
        polygon.get_polygon(hull_ids, true, false)
    }
}

#[derive(Default)]
pub struct QuickHull;

impl ConvexHullComputer for QuickHull {
    fn convex_hull(&self, polygon: &Polygon, _tracer: &mut Option<ConvexHullTracer>) -> Polygon {
        info!("Computing convex hull with the QuickHull algorithm");

        let mut hull_ids = HashSet::new();
        let mut stack = Vec::new();

        let x = polygon.lowest_rightmost_vertex().id;
        let y = polygon.highest_leftmost_vertex().id;
        let xy = polygon.get_line_segment(&x, &y).unwrap();
        let s = polygon
            .vertices()
            .into_iter()
            .filter(|v| ![x, y].contains(&v.id));

        hull_ids.insert(x);
        hull_ids.insert(y);
        trace!("Init hull: {hull_ids:?}");

        let (s1, s2): (Vec<_>, Vec<_>) = s.partition(|v| v.right(&xy));

        if !s1.is_empty() {
            trace!(v1:?=x, v2:?=y, s:?=s1.iter().map(|v| v.id).collect_vec(); "Push to stack");
            stack.push((x, y, s1));
        }
        if !s2.is_empty() {
            trace!(v1:?=y, v2:?=x, s:?=s2.iter().map(|v| v.id).collect_vec(); "Push to stack");
            stack.push((y, x, s2));
        }

        while let Some((a, b, s)) = stack.pop() {
            let ab = polygon.get_line_segment(&a, &b).unwrap();
            let c = s
                .iter()
                .max_by_key(|v| OF(ab.distance_to_vertex(v)))
                .unwrap()
                .id;
            trace!("Add to hull: {c:?}");
            hull_ids.insert(c);

            let ac = polygon.get_line_segment(&a, &c).unwrap();
            let cb = polygon.get_line_segment(&c, &b).unwrap();
            let s1 = s.iter().copied().filter(|v| v.right(&ac)).collect_vec();
            let s2 = s.iter().copied().filter(|v| v.right(&cb)).collect_vec();

            if !s1.is_empty() {
                trace!(v1:?=a, v2:?=c, s:?=s1.iter().map(|v| v.id).collect_vec(); "Push to stack");
                stack.push((a, c, s1));
            }
            if !s2.is_empty() {
                trace!(v1:?=c, v2:?=b, s:?=s2.iter().map(|v| v.id).collect_vec(); "Push to stack");
                stack.push((c, b, s2));
            }
            if stack.is_empty() {
                trace!("Stack is empty");
                break;
            }
        }

        info!("Computed convex hull with {} vertices", hull_ids.len());
        polygon.get_polygon(hull_ids, true, false)
    }
}

#[derive(Default)]
pub struct GrahamScan;

impl ConvexHullComputer for GrahamScan {
    fn convex_hull(&self, polygon: &Polygon, tracer: &mut Option<ConvexHullTracer>) -> Polygon {
        let mut stack = Vec::new();
        let mut vertices = polygon.min_angle_sorted_vertices(None, None);

        // Add rightmost lowest vertex and the next min-angle vertex
        // to stack to create initial line segment, both guaranteed
        // to be extreme based on vertices being sorted/cleaned
        stack.push(polygon.rightmost_lowest_vertex().id);
        stack.push(vertices.remove(0).id);

        if let Some(t) = tracer.as_mut() {
            t.steps.push(ConvexHullTracerStep {
                hull: stack.clone(),
                ..Default::default()
            });
        }

        for v in vertices.iter() {
            // If current vertex is a left turn from current segment off
            // top of stack, add vertex to incremental hull on stack and
            // continue to next vertex. Otherwise the current hull on
            // stack is wrong, continue popping until it's corrected.
            loop {
                assert!(stack.len() >= 2);
                let v_top = stack[stack.len() - 1];
                let v_prev = stack[stack.len() - 2];
                let ls = polygon.get_line_segment(&v_prev, &v_top).unwrap();
                if v.left(&ls) {
                    stack.push(v.id);
                } else {
                    stack.pop();
                }

                if let Some(t) = tracer.as_mut() {
                    t.steps.push(ConvexHullTracerStep {
                        hull: stack.clone(),
                        next_vertex: Some(v.id),
                        ..Default::default()
                    });
                }

                if stack[stack.len() - 1] == v.id {
                    break;
                }
            }
        }

        // The stack at the end has all hull vertices
        polygon.get_polygon(stack, true, false)
    }
}

#[derive(Default)]
pub struct DivideConquer;

impl DivideConquer {
    fn lower_tangent_vertices<'a>(
        &'a self,
        left: impl Geometry,
        right: impl Geometry,
        polygon: &'a Polygon,
    ) -> (VertexId, VertexId) {
        let mut a = left.lowest_rightmost_vertex().id;
        let mut b = right.lowest_leftmost_vertex().id;
        let mut lt = polygon.get_line_segment(&a, &b).unwrap();
        trace!(a_0:?=a, b_0:?=b, lt_0:?=lt; "Searching for lower tangent vertices");
        while !lt.is_lower_tangent(&a, &left) || !lt.is_lower_tangent(&b, &right) {
            trace!("Moving left vertex down CW until lower tangent");
            while !lt.is_lower_tangent(&a, &left) {
                a = left.prev_vertex_id(&a).unwrap(); // Move down cw
                lt = polygon.get_line_segment(&a, &b).unwrap();
                trace!(a:?, lt:?; "Step");
            }
            trace!("Left lower tangent satisfied");
            trace!("Moving right vertex up CCW until lower tangent");
            while !lt.is_lower_tangent(&b, &right) {
                b = right.next_vertex_id(&b).unwrap(); // Move down ccw
                lt = polygon.get_line_segment(&a, &b).unwrap();
                trace!(b:?, lt:?; "Step");
            }
            trace!("Right lower tangent satisfied");
        }
        trace!("Lower tangent vertices: {a}, {b}");
        (a, b)
    }

    fn upper_tangent_vertices<'a>(
        &'a self,
        left: impl Geometry,
        right: impl Geometry,
        polygon: &'a Polygon,
    ) -> (VertexId, VertexId) {
        let mut a = left.highest_rightmost_vertex().id;
        let mut b = right.highest_leftmost_vertex().id;
        let mut ut = polygon.get_line_segment(&a, &b).unwrap();
        trace!(a_0:?=a, b_0:?=b, ut_0:?=ut; "Searching for upper tangent vertices");
        while !ut.is_upper_tangent(&a, &left) || !ut.is_upper_tangent(&b, &right) {
            trace!("Moving left vertex up CCW until upper tangent");
            while !ut.is_upper_tangent(&a, &left) {
                a = left.next_vertex_id(&a).unwrap(); // Move up ccw
                ut = polygon.get_line_segment(&a, &b).unwrap();
                trace!(a:?, ut:?; "Step");
            }
            trace!("Left upper tangent satisfied");
            trace!("Moving right vertex down CW until upper tangent");
            while !ut.is_upper_tangent(&b, &right) {
                b = right.prev_vertex_id(&b).unwrap(); // Move down cw
                ut = polygon.get_line_segment(&a, &b).unwrap();
                trace!(b:?, ut:?; "Step");
            }
            trace!("Right upper tangent satisfied");
        }
        trace!("Upper tangent vertices: {a}, {b}");
        (a, b)
    }

    fn extract_boundary(
        &self,
        a: impl Geometry,
        b: impl Geometry,
        lt_a: VertexId,
        lt_b: VertexId,
        ut_a: VertexId,
        ut_b: VertexId,
    ) -> Vec<VertexId> {
        trace!(lt_a:?, lt_b:?, ut_a:?, ut_b:?; "Extracting boundary");

        // Combine into one polygon by connecting the vertex chains
        // of B -> ut -> A -> lt excluding vertices that do not
        // exist along the outer boundary
        let mut boundary = Vec::new();
        // Extract vertices from chain on B
        trace!("Starting at lower tangent vertex on B: {lt_b}");
        let mut v = lt_b;
        while v != ut_b {
            boundary.push(v);
            trace!("Boundary vertex: {v}");
            v = b.next_vertex_id(&v).unwrap();
        }
        trace!("Reached upper tangent vertex on B: {ut_b}");
        boundary.push(ut_b);
        // Extract vertices from chain on A
        trace!("Adding upper tangent vertex on A: {ut_a}");
        v = ut_a;
        while v != lt_a {
            boundary.push(v);
            trace!("Boundary vertex: {v}");
            v = a.next_vertex_id(&v).unwrap();
        }
        boundary.push(lt_a);
        trace!("Reached lower tangent vertex on A: {lt_a}");
        trace!("Final boundary: {boundary:?}");
        boundary
    }

    fn merge_from_tangents<'a>(
        &'a self,
        left: impl Geometry,
        right: impl Geometry,
        polygon: &'a Polygon,
    ) -> Vec<VertexId> {
        let (lt_a, lt_b) = self.lower_tangent_vertices(&left, &right, polygon);
        let (ut_a, ut_b) = self.upper_tangent_vertices(&left, &right, polygon);
        self.extract_boundary(&left, &right, lt_a, lt_b, ut_a, ut_b)
    }

    fn clean_triangle_ids(&self, ids: &mut Vec<VertexId>, polygon: &Polygon) {
        let triangle = polygon.get_triangle(&ids[0], &ids[1], &ids[2]).unwrap();
        if triangle.area() < 0.0 {
            ids.reverse();
        } else if triangle.area() == 0.0 {
            // Collinear, remove middle vertex
            *ids = vec![
                triangle.lowest_leftmost_vertex().id,
                triangle.highest_rightmost_vertex().id,
            ];
        }
    }

    fn merge(
        &self,
        mut left_ids: Vec<VertexId>,
        mut right_ids: Vec<VertexId>,
        polygon: &Polygon,
    ) -> Vec<VertexId> {
        trace!("Merging {left_ids:?} and {right_ids:?}");

        let merged_ids;

        if right_ids.len() == 3 {
            self.clean_triangle_ids(&mut right_ids, polygon);
        }
        if left_ids.len() == 3 {
            self.clean_triangle_ids(&mut left_ids, polygon);
        }

        if right_ids.len() >= 3 && left_ids.len() >= 3 {
            let right = polygon.get_polygon(right_ids, false, false);
            let left = polygon.get_polygon(left_ids, false, false);
            merged_ids = self.merge_from_tangents(left, right, polygon);
        } else if left_ids.len() >= 3 {
            assert!(right_ids.len() == 2);
            let left = polygon.get_polygon(left_ids, false, false);
            let right = polygon
                .get_line_segment(&right_ids[0], &right_ids[1])
                .unwrap();
            merged_ids = self.merge_from_tangents(left, right, polygon);
        } else if right_ids.len() >= 3 {
            assert!(left_ids.len() == 2);
            let right = polygon.get_polygon(right_ids, false, false);
            let left = polygon
                .get_line_segment(&left_ids[0], &left_ids[1])
                .unwrap();
            merged_ids = self.merge_from_tangents(left, right, polygon);
        } else {
            assert!(left_ids.len() == 2);
            assert!(right_ids.len() == 2);
            let right = polygon
                .get_line_segment(&right_ids[0], &right_ids[1])
                .unwrap();
            let left = polygon
                .get_line_segment(&left_ids[0], &left_ids[1])
                .unwrap();
            if left.collinear_with(&right) {
                trace!("Merging as collinear line segments");
                merged_ids = vec![
                    left.lowest_leftmost_vertex().id,
                    right.highest_rightmost_vertex().id,
                ];
            } else {
                merged_ids = self.merge_from_tangents(left, right, polygon);
            }
        }
        // Could be 2 if we tried to merge 2 collinear linear segments
        assert!(merged_ids.len() >= 2);
        trace!("Merged: {merged_ids:?}");
        merged_ids
    }
}

impl ConvexHullComputer for DivideConquer {
    fn convex_hull(&self, polygon: &Polygon, _tracer: &mut Option<ConvexHullTracer>) -> Polygon {
        info!("Computing convex hull with the DivideConquer algorithm");

        if polygon.num_vertices() == 3 {
            return polygon.clone();
        }

        let mut split_stack = Vec::new();
        let mut merge_stack = Vec::new();

        let ids = polygon.vertex_ids_by_increasing_x();
        trace!("Push to split stack: {ids:?}");
        split_stack.push(ids);

        while let Some(mut left_ids) = split_stack.pop() {
            assert!(left_ids.len() >= 4);
            let mut right_ids = left_ids.split_off(left_ids.len() / 2);

            if left_ids.len() <= 3 && right_ids.len() <= 3 {
                // Keep leftmost towards bottom of merge stack
                trace!("Push to merge stack: {left_ids:?}");
                trace!("Push to merge stack: {right_ids:?}");
                merge_stack.push(left_ids);
                merge_stack.push(right_ids);
            } else if left_ids.len() <= 3 {
                trace!("Push to merge stack: {left_ids:?}");
                trace!("Push to split stack: {right_ids:?}");
                merge_stack.push(left_ids);
                split_stack.push(right_ids);
            } else {
                // Keep rightmost towards bottom of split stack
                trace!("Push to split stack: {left_ids:?}");
                trace!("Push to split stack: {right_ids:?}");
                split_stack.push(right_ids);
                split_stack.push(left_ids);
            }

            while merge_stack.len() > 1 {
                right_ids = merge_stack.pop().unwrap();
                left_ids = merge_stack.pop().unwrap();
                let merged_ids = self.merge(left_ids, right_ids, polygon);
                trace!("Push to merge stack: {merged_ids:?}");
                merge_stack.push(merged_ids);
            }
        }

        let hull_ids = merge_stack
            .pop()
            .expect("Merge stack should have exactly 1 element");
        info!("Computed convex hull with {} vertices", hull_ids.len());
        polygon.get_polygon(hull_ids, true, true)
    }
}

#[derive(Default)]
pub struct Incremental;

impl Incremental {
    fn init_hull_three_leftmost(&self, polygon: &Polygon) -> (Polygon, Vec<VertexId>) {
        // Initialize hull with three leftmost vertices
        let mut hull_ids = polygon.vertex_ids_by_increasing_x();
        let other_ids = hull_ids.split_off(3);
        if polygon
            .get_triangle(&hull_ids[0], &hull_ids[1], &hull_ids[2])
            .unwrap()
            .area()
            < 0.0
        {
            debug!("Reversing init vertices to have valid CCW area");
            hull_ids.reverse();
        }
        debug!("Initial hull (three leftmost vertices): {hull_ids:?}");
        let hull = polygon.get_polygon(hull_ids, false, false);
        (hull, other_ids)
    }

    fn upper_tangent_vertex(&self, hull: &Polygon, v: VertexId, polygon: &Polygon) -> VertexId {
        let mut ut_v_id = hull.highest_rightmost_vertex().id;
        let mut ut = polygon.get_line_segment(&ut_v_id, &v).unwrap();

        trace!(
            v:?=polygon.get_vertex(&ut_v_id).unwrap(), ut:?;
            "Starting upper tangent vertex search"
        );

        let mut step = 1;
        while !ut.is_upper_tangent(&ut_v_id, &hull) {
            ut_v_id = hull.next_vertex_id(&ut_v_id).unwrap(); // Move up ccw
            ut = polygon.get_line_segment(&ut_v_id, &v).unwrap();
            trace!(v:?=polygon.get_vertex(&ut_v_id).unwrap(), ut:?; "Step {step}");
            step += 1;
        }

        ut_v_id
    }

    fn lower_tangent_vertex(&self, hull: &Polygon, v: VertexId, polygon: &Polygon) -> VertexId {
        let mut lt_v_id = hull.lowest_rightmost_vertex().id;
        let mut lt = polygon.get_line_segment(&lt_v_id, &v).unwrap();

        trace!(
            v:?=polygon.get_vertex(&lt_v_id).unwrap(), lt:?;
            "Starting lower tangent vertex search"
        );

        let mut step = 1;
        while !lt.is_lower_tangent(&lt_v_id, &hull) {
            lt_v_id = hull.prev_vertex_id(&lt_v_id).unwrap(); // Move down cw
            lt = polygon.get_line_segment(&lt_v_id, &v).unwrap();
            trace!(v:?=polygon.get_vertex(&lt_v_id).unwrap(), lt:?; "Step {step}");
            step += 1;
        }

        lt_v_id
    }

    fn extract_boundary(
        &self,
        hull: Polygon,
        new_v: VertexId,
        hull_ut_v: VertexId,
        hull_lt_v: VertexId,
    ) -> Vec<VertexId> {
        let mut boundary = vec![new_v];
        let mut v = hull_ut_v;

        trace!(new_v:?, hull_ut_v:?, hull_lt_v:?; "Extracting boundary");

        while v != hull_lt_v {
            boundary.push(v);
            v = hull.next_vertex_id(&v).unwrap();
            trace!(v:?; "Boundary vertex");
        }
        boundary.push(hull_lt_v);
        boundary
    }
}

impl ConvexHullComputer for Incremental {
    fn convex_hull(&self, polygon: &Polygon, tracer: &mut Option<ConvexHullTracer>) -> Polygon {
        info!("Computing convex hull with the Incremental algorithm");

        let polygon = polygon.clone_clean_collinear();
        let (mut hull, ids) = self.init_hull_three_leftmost(&polygon);
        if let Some(t) = tracer.as_mut() {
            t.steps.push(ConvexHullTracerStep {
                hull: hull.vertex_ids(),
                ..Default::default()
            });
        }

        for id in ids.into_iter() {
            debug!("Current ID: {id}");

            let ut_v = self.upper_tangent_vertex(&hull, id, &polygon);
            let lt_v = self.lower_tangent_vertex(&hull, id, &polygon);
            let new_hull_ids = self.extract_boundary(hull, id, ut_v, lt_v);

            debug!("Current hull: {new_hull_ids:?}");
            hull = polygon.get_polygon(new_hull_ids, false, true);

            if let Some(t) = tracer.as_mut() {
                t.steps.push(ConvexHullTracerStep {
                    next_vertex: Some(id),
                    upper_tangent_vertex: Some(ut_v),
                    lower_tangent_vertex: Some(lt_v),
                    hull: hull.vertex_ids(),
                });
            }
        }

        info!("Computed convex hull with {} vertices", hull.num_vertices());
        hull
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use env_logger;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[apply(convex_hull_cases)]
    fn test_convex_hull(
        #[case] case: PolygonTestCase,
        #[values(
            DivideConquer,
            ExtremeEdges,
            GiftWrapping,
            GrahamScan,
            Incremental,
            // Can enable this but it's slow AF for large number of vertices
            // InteriorPoints,
            QuickHull
        )]
        computer: impl ConvexHullComputer,
    ) {
        let _ = env_logger::builder().is_test(true).try_init();
        let hull = computer.convex_hull(&case.polygon, &mut None);
        let hull_ids = hull.vertex_ids().into_iter().sorted().collect_vec();
        assert_eq!(hull_ids, case.metadata.extreme_points);
    }
}
