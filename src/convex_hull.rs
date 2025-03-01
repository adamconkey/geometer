use std::collections::HashSet;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::{line_segment::LineSegment, polygon::Polygon, vertex::VertexId};


#[derive(Debug)]
pub struct ConvexHull {
    vertices: HashSet<VertexId>,
}

impl ConvexHull {
    pub fn new(vertices: HashSet<VertexId>) -> Self {
        ConvexHull { vertices }
    }

    pub fn add_vertex(&mut self, vertex_id: VertexId) {
        self.vertices.insert(vertex_id);
    }

    pub fn add_vertices(&mut self, vertices: impl IntoIterator<Item = VertexId>) {
        for v in vertices {
            self.add_vertex(v);
        }
    }

    pub fn get_vertices(&self) -> Vec<VertexId> {
        // TODO this is a little silly to sort on every retrieval 
        // especially if nothing changes. I originally had a 
        // min-heap but found you still had to clone/sort to get 
        // the whole vec (not just peek/pop from top), so just 
        // doing vec seemed simpler, then I added lazy sorting
        // but realized if going that route, should just maintain
        // sort order on input. Long story short there are smarter
        // ways to do this but this is simple and fast, should
        // do this smarter at a later time. 
        let mut sorted = self.vertices.iter().cloned().collect_vec();
        sorted.sort();
        sorted
    }
}

impl PartialEq for ConvexHull {
    fn eq(&self, other: &Self) -> bool {
        self.get_vertices() == other.get_vertices()
    }
}

impl Default for ConvexHull {
    fn default() -> Self {
        let vertices = HashSet::new();
        ConvexHull { vertices }
    }
}


pub trait ConvexHullComputer {
    fn convex_hull(&self, polgyon: &Polygon) -> ConvexHull;
}


#[derive(Default)]
pub struct GiftWrapping;


impl ConvexHullComputer for GiftWrapping {
    fn convex_hull(&self, polygon: &Polygon) -> ConvexHull {
        let mut hull = ConvexHull::default();
        // Form a horizontal line terminating at lowest point to start
        let v0 = polygon.leftmost_lowest_vertex();
        let mut p = v0.coords.clone();
        p.x -= 1.0;  // Arbitrary distance
        let mut current_edge = LineSegment::new(&p, &v0.coords);
        let mut current_vertex_id = v0.id;
        hull.add_vertex(current_vertex_id);

        // Perform gift-wrapping, using the previous hull edge as a vector to 
        // find the point with the least CCW angle w.r.t. the vector. Connect 
        // that point to the current terminal vertex to form the newest hull 
        // edge. Repeat until we reach the starting vertex again.
        loop {
            let min_angle_vertex_id = polygon.vertices()
                .iter()
                .filter(|v| v.id != current_vertex_id)
                .min_by_key(|v| OrderedFloat(current_edge.angle_to_point(&v.coords)))
                .unwrap()
                .id;

            current_edge = polygon.get_line_segment(
                &current_vertex_id, &min_angle_vertex_id
            ).unwrap();
            current_vertex_id = min_angle_vertex_id;
            if current_vertex_id == v0.id {
                break;
            } else {
                hull.add_vertex(current_vertex_id);
            }

        }
        hull
    }
}


#[derive(Default)]
pub struct QuickHull;


impl ConvexHullComputer for QuickHull {

    fn convex_hull(&self, polygon: &Polygon) -> ConvexHull {
        let mut hull = ConvexHull::default();
        let mut stack = Vec::new();

        let x = polygon.lowest_rightmost_vertex().id;
        let y = polygon.highest_leftmost_vertex().id;
        let xy = polygon.get_line_segment(&x, &y).unwrap();
        let s = polygon.vertices()
            .into_iter()
            .filter(|v| v.id != x && v.id != y)
            .collect_vec();

        hull.add_vertex(x);
        hull.add_vertex(y);

        let (s1, s2): (Vec<_>, Vec<_>) = s
            .into_iter()
            .partition(|v| v.right(&xy));

        if !s1.is_empty() { stack.push((x, y, s1)) };
        if !s2.is_empty() { stack.push((y, x, s2)) };

        loop {
            let (a, b, s) = stack.pop().unwrap();
            let ab = polygon.get_line_segment(&a, &b).unwrap();

            let c = s.iter()
                .max_by_key(|v| OrderedFloat(ab.distance_to_point(&v.coords)))
                .unwrap()
                .id;
            hull.add_vertex(c);

            let ac = polygon.get_line_segment(&a, &c).unwrap();
            let cb = polygon.get_line_segment(&c, &b).unwrap();

            let s1 = s.iter()
                .copied()
                .filter(|v| v.right(&ac))
                .collect_vec();

            let s2 = s.iter()
                .copied()
                .filter(|v| v.right(&cb))
                .collect_vec();

            if !s1.is_empty() { stack.push((a, c, s1)); }
            if !s2.is_empty() { stack.push((c, b, s2)); }
            if stack.is_empty() { break; }
        }
        hull
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{self, *};
    use crate::test_util::*;

    // TODO doing it this way, can you also parametrize on algorithm?

    #[apply(extreme_point_cases)]
    fn test_convex_hull_from_gift_wrapping(#[case] case: PolygonTestCase) {
        let computer = GiftWrapping::default();
        let hull = computer.convex_hull(&case.polygon);
        let expected_hull = ConvexHull::new(case.metadata.extreme_points);
        assert_eq!(hull, expected_hull);
    }

    #[apply(extreme_point_cases)]
    fn test_convex_hull_from_quick_hull(#[case] case: PolygonTestCase) {
        let computer = QuickHull::default();
        let hull = computer.convex_hull(&case.polygon);
        let expected_hull = ConvexHull::new(case.metadata.extreme_points);
        assert_eq!(hull, expected_hull);
    }

}