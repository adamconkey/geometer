use crate::line_segment::LineSegment;

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

impl From<&LineSegment<'_>> for Vector {
    fn from(ls: &LineSegment) -> Self {
        Vector::new(ls.v2.x - ls.v1.x, ls.v2.y - ls.v1.y)
    }
}
