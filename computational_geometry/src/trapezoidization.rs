

// TODO need to determine what the data structures are here.
// The trapezoid in this context I think can fundamentally
// be defined by refs to two edges (line segments), where
// one is a primary edge that has the highest y-coordinate
// vertex and the other is secondary. From that you can
// then construct a trapezoid using intersections of the
// horizontals with the edges. 

use std::collections::HashSet;

use crate::line_segment::LineSegment;

pub struct Trapezoid<'a> {
    e1: LineSegment<'a>,
    e2: LineSegment<'a>,
}


pub struct Trapezoidization<'a> {
    trapezoids: HashSet<Trapezoid<'a>>,
}