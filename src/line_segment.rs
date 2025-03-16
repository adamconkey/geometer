use std::collections::HashSet;

use crate::{
    geometry::Geometry,
    triangle::Triangle,
    vector::Vector,
    vertex::{Vertex, VertexId},
};

#[derive(Clone, Debug, PartialEq)]
pub struct LineSegment {
    pub v1: Vertex,
    pub v2: Vertex,
}

impl Geometry for LineSegment {
    fn vertices(&self) -> Vec<&Vertex> {
        vec![&self.v1, &self.v2]
    }

    fn edges(&self) -> HashSet<(VertexId, VertexId)> {
        let mut edges = HashSet::new();
        edges.insert((self.v1.id, self.v2.id));
        edges
    }
}

impl LineSegment {
    pub fn from_vertices(v1: Vertex, v2: Vertex) -> Self {
        LineSegment { v1, v2 }
    }

    pub fn reverse(&self) -> LineSegment {
        LineSegment::from_vertices(self.v2.clone(), self.v1.clone())
    }

    pub fn is_vertical(&self) -> bool {
        self.v1.x == self.v2.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.v1.y == self.v2.y
    }

    pub fn proper_intersects(&self, cd: &LineSegment) -> bool {
        let a = &self.v1;
        let b = &self.v2;
        let c = &cd.v1;
        let d = &cd.v2;

        let abc = Triangle::from_vertices(a.clone(), b.clone(), c.clone());
        let abd = Triangle::from_vertices(a.clone(), b.clone(), d.clone());
        let cda = Triangle::from_vertices(c.clone(), d.clone(), a.clone());
        let cdb = Triangle::from_vertices(c.clone(), d.clone(), b.clone());

        let has_collinear_points = abc.has_collinear_points()
            || abd.has_collinear_points()
            || cda.has_collinear_points()
            || cdb.has_collinear_points();

        if has_collinear_points {
            return false;
        }

        let ab_splits_cd = c.left(self) ^ d.left(self);
        let cd_splits_ab = a.left(cd) ^ b.left(cd);
        ab_splits_cd && cd_splits_ab
    }

    pub fn improper_intersects(&self, cd: &LineSegment) -> bool {
        let a = &self.v1;
        let b = &self.v2;
        let c = &cd.v1;
        let d = &cd.v2;

        c.between(a, b) || d.between(a, b) || a.between(c, d) || b.between(c, d)
    }

    pub fn intersects(&self, cd: &LineSegment) -> bool {
        self.proper_intersects(cd) || self.improper_intersects(cd)
    }

    pub fn connected_to(&self, cd: &LineSegment) -> bool {
        self.incident_to(&cd.v1) || self.incident_to(&cd.v2)
    }

    pub fn incident_to(&self, v: &Vertex) -> bool {
        self.v1.coords() == v.coords() || self.v2.coords() == v.coords()
    }

    pub fn length(&self) -> f64 {
        Vector::from(self).magnitude()
    }

    pub fn angle_to_vertex(&self, v: &Vertex) -> f64 {
        // TODO this is a specific interpretation of "angle to point"
        // which is between two vectors p1 -> p2 and p2 -> p. Not
        // sure it's the best named function but it's nice to have
        // the function as it makes sorting vecs by the value very
        // compact. Can consider moving to another struct or renaming
        let p1_to_p2 = Vector::from(self);
        let p2_to_p = Vector::from(&LineSegment::from_vertices(self.v2.clone(), v.clone()));
        let cos_theta = p1_to_p2.dot(&p2_to_p) / (p1_to_p2.magnitude() * p2_to_p.magnitude());
        cos_theta.acos()
    }

    pub fn distance_to_vertex(&self, p: &Vertex) -> f64 {
        // https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
        let p1_to_p2 = Vector::from(self);
        let t1 = p1_to_p2.y * p.x;
        let t2 = p1_to_p2.x * p.y;
        let t3 = self.v2.x * self.v1.y;
        let t4 = self.v2.y * self.v1.x;
        (t1 - t2 + t3 - t4).abs() / p1_to_p2.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_intersect() {
        let a = Vertex::new(VertexId::from(0u32), 6.0, 4.0);
        let b = Vertex::new(VertexId::from(1u32), 0.0, 4.0);
        let c = Vertex::new(VertexId::from(2u32), 1.0, 0.0);
        let d = Vertex::new(VertexId::from(3u32), 4.0, 6.0);

        let ab = LineSegment::from_vertices(a, b);
        let cd = LineSegment::from_vertices(c, d);

        assert!(ab.proper_intersects(&cd));
        assert!(cd.proper_intersects(&ab));
        assert!(!ab.improper_intersects(&cd));
        assert!(!cd.improper_intersects(&ab));
        assert!(ab.intersects(&cd));
        assert!(cd.intersects(&ab));
    }

    #[test]
    fn test_improper_intersect() {
        let a = Vertex::new(VertexId::from(0u32), 6.0, 6.0);
        let b = Vertex::new(VertexId::from(1u32), 0.0, 6.0);
        let c = Vertex::new(VertexId::from(2u32), 1.0, 0.0);
        let d = Vertex::new(VertexId::from(3u32), 4.0, 6.0);

        let ab = LineSegment::from_vertices(a, b);
        let cd = LineSegment::from_vertices(c, d);

        assert!(!ab.proper_intersects(&cd));
        assert!(!cd.proper_intersects(&ab));
        assert!(ab.improper_intersects(&cd));
        assert!(cd.improper_intersects(&ab));
        assert!(ab.intersects(&cd));
        assert!(cd.intersects(&ab));
    }

    #[test]
    fn test_no_intersect() {
        let a = Vertex::new(VertexId::from(0u32), 6.0, 4.0);
        let b = Vertex::new(VertexId::from(1u32), 4.0, 4.0);
        let c = Vertex::new(VertexId::from(2u32), 1.0, 0.0);
        let d = Vertex::new(VertexId::from(3u32), 4.0, 6.0);

        let ab = LineSegment::from_vertices(a, b);
        let cd = LineSegment::from_vertices(c, d);

        assert!(!ab.proper_intersects(&cd));
        assert!(!cd.proper_intersects(&ab));
        assert!(!ab.improper_intersects(&cd));
        assert!(!cd.improper_intersects(&ab));
        assert!(!ab.intersects(&cd));
        assert!(!cd.intersects(&ab));
    }

    #[test]
    fn test_intersect_with_self() {
        let a = Vertex::new(VertexId::from(0u32), 6.0, 4.0);
        let b = Vertex::new(VertexId::from(1u32), 4.0, 4.0);
        let ab = LineSegment::from_vertices(a, b);

        assert!(!ab.proper_intersects(&ab));
        assert!(ab.improper_intersects(&ab));
        assert!(ab.intersects(&ab));
    }

    #[test]
    fn test_reverse() {
        let a = Vertex::new(VertexId::from(0u32), 0.0, 0.0);
        let b = Vertex::new(VertexId::from(1u32), 1.0, 2.0);
        let ab = LineSegment::from_vertices(a.clone(), b.clone());
        let ba = ab.reverse();
        assert_eq!(ab.v1.coords(), a.coords());
        assert_eq!(ab.v2.coords(), b.coords());
        assert_eq!(ba.v1.coords(), b.coords());
        assert_eq!(ba.v2.coords(), a.coords());
    }
}
