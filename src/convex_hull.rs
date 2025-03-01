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



#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{self, *};
    use crate::test_util::*;

    #[apply(extreme_point_cases)]
    fn test_convex_hull_from_gift_wrapping(#[case] case: PolygonTestCase) {
        let computer = GiftWrapping::default();
        let hull = computer.convex_hull(&case.polygon);
        let expected_hull = ConvexHull::new(case.metadata.extreme_points);
        assert_eq!(hull, expected_hull);
    }

}