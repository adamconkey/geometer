use crate::{line_segment::LineSegment, point::Point};

// TODO I am unlikely to keep this as a primitive, I will 
// probably farm this out to an external linear algebra
// library but am not convinced yet as to what that
// dependency should be
pub struct Vector {
    pub x: f64,
    pub y: f64,
}


impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


impl<'a> From<&'a LineSegment<'a>> for Vector {
    fn from(ls: &'a LineSegment<'a>) -> Self {
        Vector::new(ls.p2.x - ls.p1.x, ls.p2.y - ls.p1.y)
    }
}