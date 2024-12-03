

// TODO implement logic to determine actual polygon
// shape from edges, probably that can be run in the
// new function on creation? Create tests to validate.

use std::collections::HashSet;

use crate::line_segment::LineSegment;

pub struct Trapezoid<'a> {
    e1: LineSegment<'a>,
    e2: LineSegment<'a>,
}


pub struct Trapezoidization<'a> {
    trapezoids: HashSet<Trapezoid<'a>>,
}