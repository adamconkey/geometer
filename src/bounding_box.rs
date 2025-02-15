use crate::point::Point;

pub struct BoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}


impl BoundingBox {
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self{
        Self { min_x, max_x, min_y, max_y }
    }

    pub fn center(&self) -> Point {
        let x = 0.5 * (self.max_x - self.min_x) + self.min_x;
        let y = 0.5 * (self.max_y - self.min_y) + self.min_y;
        Point::new(x, y)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // TODO could add some more cases to this one
    #[test]
    fn test_center() {
        let bb = BoundingBox::new(0.0, 10.0, 0.0, 6.0);
        let expected_center = Point::new(5.0, 3.0);
        assert_eq!(bb.center(), expected_center);
    }
}