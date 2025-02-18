use crate::line_segment::LineSegment;


#[derive(Debug, PartialEq)]
pub struct ConvexHull {
    pub edges: Vec<LineSegment>,
}


impl ConvexHull {
    pub fn new() -> Self {
        let edges = Vec::new();
        ConvexHull { edges }
    }
}