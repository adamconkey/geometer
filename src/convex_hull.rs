use crate::line_segment::LineSegment;


#[derive(Debug, PartialEq)]
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
    pub fn new() -> Self {
        let edges = Vec::new();
        ConvexHull { edges }
    }
}