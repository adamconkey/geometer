use serde::{Deserialize, Serialize};

use crate::{
    line_segment::LineSegment,
    triangle::Triangle,
};


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}


impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn between(&self, a: &Point, b: &Point) -> bool {
        if !Triangle::new(a, b, self).has_collinear_points() {
            return false;
        }

        let (e1, e2, check) = match LineSegment::new(a, b).is_vertical() {
            true  => (a.y, b.y, self.y),
            false => (a.x, b.x, self.x),
        };
        
        (e1..e2).contains(&check) || (e2..e1).contains(&check)
    }

    pub fn left(&self, ab: &LineSegment) -> bool {
        Triangle::new(ab.p1, ab.p2, self).area() > 0.0
    }

    pub fn left_on(&self, ab: &LineSegment) -> bool {
        Triangle::new(ab.p1, ab.p2, self).area() >= 0.0
    }

    pub fn rotate_about_origin(&mut self, radians: f64) {
        todo!("Need to implement rotation equations for point")
    }

    pub fn rotate_about_point(&mut self, radians: f64, point: Point) {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        let x_diff = self.x - point.x;
        let y_diff = self.y - point.y;
        // let x1 = x_diff * cos_theta - y_diff * sin_theta + point.x;
        todo!("Need to fix types on point to support this (i32 -> f64)")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_serialize_point() {
        let p = Point::new(1.0, 2.0);
        let serialized = serde_json::to_string(&p).unwrap();
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, p);
    }

    #[test]
    fn test_serialize_points_vec() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let points = vec![p1, p2];
        let serialized = serde_json::to_string(&points).unwrap();
        let deserialized: Vec<Point> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, points);
    }

    #[test]
    fn test_between() {
        let p0 = Point::new(0.0, 0.0);
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 2.0);

        assert!( p1.between(&p0, &p2));
        assert!( p1.between(&p2, &p1));
        assert!(!p0.between(&p1, &p2));
        assert!(!p0.between(&p2, &p1));
        assert!(!p2.between(&p0, &p1));
        assert!(!p2.between(&p1, &p0));
    }

    #[test]
    fn test_between_vertical() {
        let p0 = Point::new(0.0, 0.0);
        let p1 = Point::new(0.0, 1.0);
        let p2 = Point::new(0.0, 2.0);

        assert!( p1.between(&p0, &p2));
        assert!( p1.between(&p2, &p1));
        assert!(!p0.between(&p1, &p2));
        assert!(!p0.between(&p2, &p1));
        assert!(!p2.between(&p0, &p1));
        assert!(!p2.between(&p1, &p0));
    }
}