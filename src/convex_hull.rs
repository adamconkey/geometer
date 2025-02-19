use serde::{Deserialize, Serialize};

use crate::line_segment::LineSegment;


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ConvexHull {
    pub edges: Vec<LineSegment>,
}


// TODO want to add deserialize so that it can be read
// from JSON, and then start adding hulls to the test
// case metadata. Initially can assume ordered even
// though I think generally that won't be the case, 
// although having vec is still maybe useful from a
// visualization perspective that the order retains
// the creation order


impl ConvexHull {
    pub fn new(edges: Vec<LineSegment>) -> Self {
        ConvexHull { edges }
    }

    pub fn default() -> Self {
        let edges = Vec::new();
        Self::new(edges)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::point::Point;

    #[test]
    fn test_serialize_hull() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let p3 = Point::new(5.0, 6.0);
        let p4 = Point::new(7.0, 8.0);
        let e1 = LineSegment::new(p1, p2);
        let e2 = LineSegment::new(p3, p4);
        let edges = vec![e1, e2];
        let hull = ConvexHull::new(edges);
        let serialized = serde_json::to_string(&hull).unwrap();
        println!("SERIALZED: {:?}", serialized);
        let deserialized: ConvexHull = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, hull);
    }
}