use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{line_segment::LineSegment, triangle::Triangle, vector::Vector};

#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VertexId(u32);

impl From<u32> for VertexId {
    fn from(raw: u32) -> Self {
        Self(raw)
    }
}

impl From<usize> for VertexId {
    fn from(raw: usize) -> Self {
        Self(raw.try_into().unwrap())
    }
}

impl fmt::Display for VertexId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for VertexId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Vertex {
    pub id: VertexId,
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ id: {}, x: {}, y: {} }}", self.id, self.x, self.y)
    }
}

impl fmt::Debug for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ id: {}, x: {}, y: {} }}", self.id, self.x, self.y)
    }
}

impl Vertex {
    pub fn new(id: VertexId, x: f64, y: f64) -> Self {
        Self { id, x, y }
    }

    pub fn coords(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn between(&self, a: &Vertex, b: &Vertex) -> bool {
        if !Triangle::from_vertices(a, b, self).has_collinear_points() {
            return false;
        }

        let (e1, e2, check) = match LineSegment::from_vertices(a, b).is_vertical() {
            true => (a.y, b.y, self.y),
            false => (a.x, b.x, self.x),
        };

        (e1..e2).contains(&check) || (e2..e1).contains(&check)
    }

    pub fn left(&self, ab: &LineSegment) -> bool {
        Triangle::from_vertices(ab.v1, ab.v2, self).area() > 0.0
    }

    pub fn left_on(&self, ab: &LineSegment) -> bool {
        Triangle::from_vertices(ab.v1, ab.v2, self).area() >= 0.0
    }

    pub fn right(&self, ab: &LineSegment) -> bool {
        !self.left_on(ab)
    }

    pub fn right_on(&self, ab: &LineSegment) -> bool {
        !self.left(ab)
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn rotate_about_origin(&mut self, radians: f64) {
        let origin = Vertex::new(VertexId::default(), 0.0, 0.0);
        self.rotate_about_vertex(radians, &origin);
    }

    pub fn rotate_about_vertex(&mut self, radians: f64, v: &Vertex) {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        let x_diff = self.x - v.x;
        let y_diff = self.y - v.y;
        let x1 = x_diff * cos_theta - y_diff * sin_theta + v.x;
        let y1 = x_diff * sin_theta + y_diff * cos_theta + v.y;
        self.x = x1;
        self.y = y1;
    }

    pub fn round_coordinates(&mut self) {
        self.x = f64::round(self.x);
        self.y = f64::round(self.y);
    }

    pub fn distance_to(&self, v: &Vertex) -> f64 {
        let vec = Vector::new(v.x - self.x, v.y - self.y);
        vec.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use crate::F64_ASSERT_PRECISION;

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rstest::rstest;
    use rstest_reuse::{self, *};
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI, SQRT_2};

    #[test]
    fn test_serialize_vertex() {
        let v = Vertex::new(VertexId::default(), 0.0, 2.0);
        let serialized = serde_json::to_string(&v).unwrap();
        let deserialized: Vertex = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, v);
    }

    #[test]
    fn test_serialize_vertex_vec() {
        let p1 = Vertex::new(VertexId::from(0u32), 0.0, 2.0);
        let p2 = Vertex::new(VertexId::from(1u32), 0.0, 4.0);
        let points = vec![p1, p2];
        let serialized = serde_json::to_string(&points).unwrap();
        let deserialized: Vec<Vertex> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, points);
    }

    #[test]
    fn test_between() {
        let v0 = Vertex::new(VertexId::from(0u32), 0.0, 0.0);
        let v1 = Vertex::new(VertexId::from(1u32), 1.0, 1.0);
        let v2 = Vertex::new(VertexId::from(2u32), 2.0, 2.0);

        assert!(v1.between(&v0, &v2));
        assert!(v1.between(&v2, &v1));
        assert!(!v0.between(&v1, &v2));
        assert!(!v0.between(&v2, &v1));
        assert!(!v2.between(&v0, &v1));
        assert!(!v2.between(&v1, &v0));
    }

    #[test]
    fn test_between_vertical() {
        let v0 = Vertex::new(VertexId::from(0u32), 0.0, 0.0);
        let v1 = Vertex::new(VertexId::from(1u32), 0.0, 1.0);
        let v2 = Vertex::new(VertexId::from(2u32), 0.0, 2.0);

        assert!(v1.between(&v0, &v2));
        assert!(v1.between(&v2, &v1));
        assert!(!v0.between(&v1, &v2));
        assert!(!v0.between(&v2, &v1));
        assert!(!v2.between(&v0, &v1));
        assert!(!v2.between(&v1, &v0));
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
        let mut v = Vertex::new(VertexId::default(), 1.0, 0.0);
        v.rotate_about_origin(radians);
        assert_approx_eq!(v.x, x, F64_ASSERT_PRECISION);
        assert_approx_eq!(v.y, y, F64_ASSERT_PRECISION);
    }

    #[apply(unit_circle_rotations)]
    fn test_rotate_about_self(radians: f64, _x: f64, _y: f64) {
        let mut v = Vertex::new(VertexId::default(), 1.0, 2.0);
        v.rotate_about_vertex(radians, &v.clone());
        assert_approx_eq!(v.x, v.x, F64_ASSERT_PRECISION);
        assert_approx_eq!(v.y, v.y, F64_ASSERT_PRECISION);
    }

    #[test]
    fn test_rotation_composition() {
        let mut v1 = Vertex::new(VertexId::default(), 1.0, 0.0);
        let mut v2 = v1.clone();
        v1.rotate_about_origin(FRAC_PI_4);
        v1.rotate_about_origin(FRAC_PI_4);
        v2.rotate_about_origin(FRAC_PI_2);
        assert_approx_eq!(v1.x, v2.x, F64_ASSERT_PRECISION);
        assert_approx_eq!(v1.y, v2.y, F64_ASSERT_PRECISION);
    }

    // TODO need tests for rotation about arbitrary point
}
