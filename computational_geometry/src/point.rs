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
        let origin = Point::new(0.0, 0.0);
        self.rotate_about_point(radians, &origin);
    }

    pub fn rotate_about_point(&mut self, radians: f64, point: &Point) {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        let x_diff = self.x - point.x;
        let y_diff = self.y - point.y;
        let x1 = x_diff * cos_theta - y_diff * sin_theta + point.x;
        let y1 = x_diff * sin_theta + y_diff * cos_theta + point.y;
        self.x = x1;
        self.y = y1;
    }
}


#[cfg(test)]
mod tests {
    use crate::F64_ASSERT_PRECISION;

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rstest_reuse::{self, *};
    use rstest::rstest;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI, SQRT_2};
 
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

    #[template]
    #[rstest]
    #[case(0.0, 1.0, 0.0)]
    #[case(FRAC_PI_8, 0.5 * (2.0 + SQRT_2).sqrt(), 0.5 * (2.0 - SQRT_2).sqrt())]
    #[case(FRAC_PI_6, 0.5 * 3.0f64.sqrt(), 0.5)]
    #[case(FRAC_PI_4, 0.5 * SQRT_2, 0.5 * SQRT_2)]
    #[case(FRAC_PI_3, 0.5, 0.5 * 3.0f64.sqrt())]
    #[case(3.0 * FRAC_PI_8, 0.5 * (2.0 - SQRT_2).sqrt(), 0.5 * (2.0 + SQRT_2).sqrt())]
    #[case(FRAC_PI_2, 0.0, 1.0)]
    #[case(5.0 * FRAC_PI_8, -0.5 * (2.0 - SQRT_2).sqrt(), 0.5 * (2.0 + SQRT_2).sqrt())]
    #[case(2.0 * FRAC_PI_3, -0.5, 0.5 * 3.0f64.sqrt())]
    #[case(3.0 * FRAC_PI_4, -0.5 * SQRT_2, 0.5 * SQRT_2)]
    #[case(5.0 * FRAC_PI_6, -0.5 * 3.0f64.sqrt(), 0.5)]
    #[case(7.0 * FRAC_PI_8, -0.5 * (2.0 + SQRT_2).sqrt(), 0.5 * (2.0 - SQRT_2).sqrt())]
    #[case(PI, -1.0, 0.0)]
    #[case(9.0 * FRAC_PI_8, -0.5 * (2.0 + SQRT_2).sqrt(), -0.5 * (2.0 - SQRT_2).sqrt())]
    #[case(7.0 * FRAC_PI_6, -0.5 * 3.0f64.sqrt(), -0.5)]
    #[case(5.0 * FRAC_PI_4, -0.5 * SQRT_2, -0.5 * SQRT_2)]
    #[case(4.0 * FRAC_PI_3, -0.5, -0.5 * 3.0f64.sqrt())]
    #[case(11.0 * FRAC_PI_8, -0.5 * (2.0 - SQRT_2).sqrt(), -0.5 * (2.0 + SQRT_2).sqrt())]
    #[case(3.0 * FRAC_PI_2, 0.0, -1.0)]
    #[case(13.0 * FRAC_PI_8, 0.5 * (2.0 - SQRT_2).sqrt(), -0.5 * (2.0 + SQRT_2).sqrt())]
    #[case(5.0 * FRAC_PI_3, 0.5, -0.5 * 3.0f64.sqrt())]
    #[case(7.0 * FRAC_PI_4, 0.5 * SQRT_2, -0.5 * SQRT_2)]
    #[case(11.0 * FRAC_PI_6, 0.5 * 3.0f64.sqrt(), -0.5)]
    #[case(15.0 * FRAC_PI_8, 0.5 * (2.0 + SQRT_2).sqrt(), -0.5 * (2.0 - SQRT_2).sqrt())]
    #[case(2.0 * PI, 1.0, 0.0)]
    fn unit_circle_rotations(#[case] radians: f64, #[case] x: f64, #[case] y: f64) {}   

    
    #[apply(unit_circle_rotations)]
    fn test_rotate_about_origin(radians: f64, x: f64, y: f64) {
        let mut p = Point::new(1.0, 0.0);
        p.rotate_about_origin(radians);
        assert_approx_eq!(p.x, x, F64_ASSERT_PRECISION);
        assert_approx_eq!(p.y, y, F64_ASSERT_PRECISION);
    }
}