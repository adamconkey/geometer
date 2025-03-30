use itertools::Itertools;
use ordered_float::OrderedFloat as OF;
use std::{cmp::Reverse, collections::HashSet};

use crate::{
    geometry::Geometry,
    line_segment::LineSegment,
    polygon::Polygon,
    vertex::{Vertex, VertexId},
};

pub trait ConvexHullComputer {
    fn convex_hull(&self, polygon: &Polygon) -> Polygon;
}

#[derive(Default)]
pub struct InteriorPoints;

impl InteriorPoints {
    pub fn interior_points(&self, polygon: &Polygon) -> HashSet<VertexId> {
        let mut interior_points = HashSet::new();
        let ids = polygon.vertex_ids();

        // Don't be fooled by the runtime here, it's iterating over all
        // permutations, which is n! / (n-4)! = n * (n-1) * (n-2) * (n-3),
        // so it's still O(n^4), this is just more compact than 4 nested
        // for-loops.
        for perm in ids.into_iter().permutations(4) {
            // TODO instead of unwrap, return result with error
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
    fn convex_hull(&self, polygon: &Polygon) -> Polygon {
        // NOTE: This is slow O(n^4) since the interior point
        // computation being used has that runtime.
        let interior_ids = self.interior_points(polygon);
        let all_ids = HashSet::from_iter(polygon.vertex_ids());
        let hull_ids = &all_ids - &interior_ids;
        polygon.get_polygon(hull_ids, true)
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
            // TODO instead of unwrap, return result with error
            let ls = polygon.get_line_segment(perm[0], perm[1]).unwrap();
            let mut is_extreme = true;
            for id3 in ids.iter().filter(|id| !perm.contains(id)) {
                // TODO instead of unwrap, return result with error
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

        // Have to do this cleaning step to account for collinear points in
        // the edge chain. Note the edge chain as-is could have collinear
        // points even if the polygon itself does not have collinear points.
        // If there's a chain xyz, this procedure will keep xz being the two
        // points furthest from each other no matter how many collinear
        // points exist between those two.
        self.remove_collinear(&mut extreme_edges, polygon);
        extreme_edges
    }

    fn remove_collinear(&self, edges: &mut Vec<(VertexId, VertexId)>, p: &Polygon) {
        edges.sort_by_key(|(id1, id2)| (*id1, Reverse(OF(p.distance_between(id1, id2)))));
        edges.dedup_by(|a, b| a.0 == b.0);
        edges.sort_by_key(|(id1, id2)| (*id2, Reverse(OF(p.distance_between(id1, id2)))));
        edges.dedup_by(|a, b| a.1 == b.1);
    }
}

impl ConvexHullComputer for ExtremeEdges {
    fn convex_hull(&self, polygon: &Polygon) -> Polygon {
        let mut hull_ids = HashSet::new();
        for (id1, id2) in self.extreme_edges(polygon).into_iter() {
            hull_ids.insert(id1);
            hull_ids.insert(id2);
        }
        polygon.get_polygon(hull_ids, true)
    }
}

#[derive(Default)]
pub struct GiftWrapping;

impl ConvexHullComputer for GiftWrapping {
    fn convex_hull(&self, polygon: &Polygon) -> Polygon {
        // Form a horizontal line terminating at lowest point to start
        let v0 = polygon.rightmost_lowest_vertex();
        let mut v = v0.clone();
        v.x -= 1.0; // Arbitrary distance
        let mut e = LineSegment::from_vertices(&v, v0);
        let mut v_i = v0;

        let mut hull_ids = HashSet::new();
        hull_ids.insert(v_i.id);

        // Perform gift-wrapping, using the previous hull edge as a vector to
        // find the point with the least CCW angle w.r.t. the vector. Connect
        // that point to the current terminal vertex to form the newest hull
        // edge. Repeat until we reach the starting vertex again.
        loop {
            let v_min_angle = polygon
                .vertices()
                .into_iter()
                .filter(|v| v.id != v_i.id)
                .sorted_by_key(|v| (OF(e.angle_to_vertex(v)), Reverse(OF(v_i.distance_to(v)))))
                .dedup_by(|a, b| e.angle_to_vertex(a) == e.angle_to_vertex(b))
                .collect::<Vec<_>>()[0];

            e = polygon.get_line_segment(&v_i.id, &v_min_angle.id).unwrap();
            v_i = v_min_angle;
            if v_i.id == v0.id {
                break;
            } else {
                hull_ids.insert(v_i.id);
            }
        }

        polygon.get_polygon(hull_ids, true)
    }
}

#[derive(Default)]
pub struct QuickHull;

impl ConvexHullComputer for QuickHull {
    fn convex_hull(&self, polygon: &Polygon) -> Polygon {
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

        let (s1, s2): (Vec<_>, Vec<_>) = s.partition(|v| v.right(&xy));

        if !s1.is_empty() {
            stack.push((x, y, s1))
        };
        if !s2.is_empty() {
            stack.push((y, x, s2))
        };

        while let Some((a, b, s)) = stack.pop() {
            let ab = polygon.get_line_segment(&a, &b).unwrap();
            let c = s
                .iter()
                .max_by_key(|v| OF(ab.distance_to_vertex(v)))
                .unwrap()
                .id;
            hull_ids.insert(c);

            let ac = polygon.get_line_segment(&a, &c).unwrap();
            let cb = polygon.get_line_segment(&c, &b).unwrap();
            let s1 = s.iter().copied().filter(|v| v.right(&ac)).collect_vec();
            let s2 = s.iter().copied().filter(|v| v.right(&cb)).collect_vec();

            if !s1.is_empty() {
                stack.push((a, c, s1));
            }
            if !s2.is_empty() {
                stack.push((c, b, s2));
            }
            if stack.is_empty() {
                break;
            }
        }

        polygon.get_polygon(hull_ids, true)
    }
}

#[derive(Default)]
pub struct GrahamScan;

impl ConvexHullComputer for GrahamScan {
    fn convex_hull(&self, polygon: &Polygon) -> Polygon {
        let mut stack = Vec::new();
        let mut vertices = polygon.min_angle_sorted_vertices();

        // Add rightmost lowest vertex and the next min-angle vertex
        // to stack to create initial line segment, both guaranteed
        // to be extreme based on vertices being sorted/cleaned
        stack.push(polygon.rightmost_lowest_vertex());
        stack.push(vertices.remove(0));

        for v in vertices.iter() {
            // If current vertex is a left turn from current segment off
            // top of stack, add vertex to incremental hull on stack and
            // continue to next vertex. Otherwise the current hull on
            // stack is wrong, continue popping until it's corrected.
            loop {
                assert!(stack.len() >= 2);
                let v_top = stack[stack.len() - 1];
                let v_prev = stack[stack.len() - 2];
                let ls = polygon.get_line_segment(&v_prev.id, &v_top.id).unwrap();
                if v.left(&ls) {
                    stack.push(v);
                    break;
                } else {
                    stack.pop();
                }
            }
        }

        // The stack at the end has all hull vertices
        polygon.get_polygon(stack.iter().map(|v| v.id), true)
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
        while !lt.is_lower_tangent(&a, &left) || !lt.is_lower_tangent(&b, &right) {
            while !lt.is_lower_tangent(&a, &left) {
                a = left.prev_vertex_id(&a).unwrap(); // Move down cw
                lt = polygon.get_line_segment(&a, &b).unwrap();
            }
            while !lt.is_lower_tangent(&b, &right) {
                b = right.next_vertex_id(&b).unwrap(); // Move down ccw
                lt = polygon.get_line_segment(&a, &b).unwrap();
            }
        }
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
        while !ut.is_upper_tangent(&a, &left) || !ut.is_upper_tangent(&b, &right) {
            while !ut.is_upper_tangent(&a, &left) {
                a = left.next_vertex_id(&a).unwrap(); // Move up ccw
                ut = polygon.get_line_segment(&a, &b).unwrap();
            }
            while !ut.is_upper_tangent(&b, &right) {
                b = right.prev_vertex_id(&b).unwrap(); // Move down cw
                ut = polygon.get_line_segment(&a, &b).unwrap();
            }
        }
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
        // Combine into one polygon by connecting the vertex chains
        // of B -> ut -> A -> lt excluding vertices that do not
        // exist along the outer boundary
        let mut boundary = Vec::new();
        // Extract vertices from chain on B
        let mut v = lt_b;
        while v != ut_b {
            boundary.push(v);
            v = b.get_next_vertex(&v).unwrap().id;
        }
        boundary.push(ut_b);
        // Extract vertices from chain on A
        v = ut_a;
        while v != lt_a {
            boundary.push(v);
            v = a.get_next_vertex(&v).unwrap().id;
        }
        boundary.push(lt_a);
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
        let merged_ids = self.extract_boundary(&left, &right, lt_a, lt_b, ut_a, ut_b);
        merged_ids
    }

    fn merge(
        &self,
        mut left_ids: Vec<VertexId>,
        mut right_ids: Vec<VertexId>,
        polygon: &Polygon,
    ) -> Vec<VertexId> {
        let merged_ids;

        // TODO I think I can implement this into triangle
        if right_ids.len() == 3 {
            let right = polygon
                .get_triangle(&right_ids[0], &right_ids[1], &right_ids[2])
                .unwrap();
            if right.area() < 0.0 {
                right_ids.reverse();
            } else if right.area() == 0.0 {
                // Collinear, remove middle vertex
                right_ids = vec![
                    right.lowest_leftmost_vertex().id,
                    right.highest_rightmost_vertex().id,
                ];
            }
        }
        if left_ids.len() == 3 {
            let left = polygon
                .get_triangle(&left_ids[0], &left_ids[1], &left_ids[2])
                .unwrap();
            if left.area() < 0.0 {
                left_ids.reverse();
            } else if left.area() == 0.0 {
                // Collinear, remove middle vertex
                left_ids = vec![
                    left.lowest_leftmost_vertex().id,
                    left.highest_rightmost_vertex().id,
                ];
            }
        }

        if right_ids.len() >= 3 && left_ids.len() >= 3 {
            let right = polygon.get_polygon(right_ids, false);
            let left = polygon.get_polygon(left_ids, false);
            merged_ids = self.merge_from_tangents(left, right, polygon);
        } else if left_ids.len() >= 3 {
            assert!(right_ids.len() == 2);
            let left = polygon.get_polygon(left_ids, false);
            let right = polygon
                .get_line_segment(&right_ids[0], &right_ids[1])
                .unwrap();
            merged_ids = self.merge_from_tangents(left, right, polygon);
        } else if right_ids.len() >= 3 {
            assert!(left_ids.len() == 2);
            let right = polygon.get_polygon(right_ids, false);
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
        merged_ids
    }
}

impl ConvexHullComputer for DivideConquer {
    fn convex_hull(&self, polygon: &Polygon) -> Polygon {
        if polygon.num_vertices() == 3 {
            return polygon.clone();
        }
        let mut split_stack = Vec::new();
        let mut merge_stack = Vec::new();

        // Sorted by increasing X and break ties by increasing Y
        let ids = polygon
            .vertex_ids()
            .iter()
            .map(|id| polygon.get_vertex(id).unwrap())
            .sorted_by_key(|v| (OF(v.x), OF(v.y)))
            .map(|v| v.id)
            .collect_vec();
        split_stack.push(ids);

        while let Some(mut left_ids) = split_stack.pop() {
            assert!(left_ids.len() >= 4);
            let mut right_ids = left_ids.split_off(left_ids.len() / 2);

            if left_ids.len() <= 3 && right_ids.len() <= 3 {
                // Keep leftmost towards bottom of merge stack
                merge_stack.push(left_ids);
                merge_stack.push(right_ids);
            } else if left_ids.len() <= 3 {
                merge_stack.push(left_ids);
                split_stack.push(right_ids);
            } else {
                // Keep rightmost towards bottom of split stack
                split_stack.push(right_ids);
                split_stack.push(left_ids);
            }

            while merge_stack.len() > 1 {
                right_ids = merge_stack.pop().unwrap();
                left_ids = merge_stack.pop().unwrap();
                let merged_ids = self.merge(left_ids, right_ids, polygon);
                merge_stack.push(merged_ids);
            }
        }

        assert!(merge_stack.len() == 1);
        let hull_ids = merge_stack.pop().unwrap();
        assert!(hull_ids.len() >= 3);
        let mut hull = polygon.get_polygon(hull_ids, true);
        hull.clean_collinear();
        hull
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
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
            // Can enable this but it's slow AF for large number of vertices
            // InteriorPoints,
            QuickHull
        )]
        computer: impl ConvexHullComputer,
    ) {
        let hull = computer.convex_hull(&case.polygon);
        let hull_ids = hull.vertex_ids().into_iter().sorted().collect_vec();
        assert_eq!(hull_ids, case.metadata.extreme_points);
    }
}
