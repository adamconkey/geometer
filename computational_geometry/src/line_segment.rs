use crate::{
    point::Point,
    triangle::Triangle,
    vertex::Vertex,
};


#[derive(Debug, PartialEq)]
pub struct LineSegment<'a> {
    pub p1: &'a Point,
    pub p2: &'a Point,
}

impl<'a> LineSegment<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point) -> Self {
        LineSegment { p1, p2 }
    }

    pub fn from_vertices(v1: &'a Vertex, v2: &'a Vertex) -> Self {
        LineSegment::new(&v1.coords, &v2.coords)
    }

    pub fn reverse(&self) -> LineSegment {
        LineSegment::new(self.p2, self.p1)
    }
    
    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn proper_intersects(&self, cd: &LineSegment) -> bool {    
        let a = self.p1;
        let b = self.p2;
        let c = cd.p1;
        let d = cd.p2;
    
        let abc = Triangle::new(a, b, c);
        let abd = Triangle::new(a, b, d);
        let cda = Triangle::new(c, d, a);
        let cdb = Triangle::new(c, d, b);
    
        let has_collinear_points =
            abc.has_collinear_points() ||
            abd.has_collinear_points() ||
            cda.has_collinear_points() ||
            cdb.has_collinear_points();
    
        if has_collinear_points {
            return false;
        }
        
        let ab_splits_cd = c.left(self) ^ d.left(self);
        let cd_splits_ab = a.left(cd) ^ b.left(cd);
        ab_splits_cd && cd_splits_ab
    }
        
    pub fn improper_intersects(&self, cd: &LineSegment) -> bool {
        let a = self.p1;
        let b = self.p2;
        let c = cd.p1;
        let d = cd.p2;
    
        c.between(a, b) || d.between(a, b) || a.between(c, d) || b.between(c, d)
    }
    
    pub fn intersects(&self, cd: &LineSegment) -> bool {
        self.proper_intersects(cd) || self.improper_intersects(cd)
    }

    pub fn connected_to(&self, cd: &LineSegment) -> bool {
        self.incident_to(cd.p1) || self.incident_to(cd.p2)
    }

    pub fn incident_to(&self, p: &Point) -> bool {
        self.p1 == p || self.p2 == p
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_proper_intersect() {
        let a = Point::new(6.0, 4.0);
        let b = Point::new(0.0, 4.0);
        let c = Point::new(1.0, 0.0);
        let d = Point::new(4.0, 6.0);

        let ab = LineSegment::new(&a, &b);
        let cd = LineSegment::new(&c, &d);

        assert!( ab.proper_intersects(&cd));
        assert!( cd.proper_intersects(&ab));
        assert!(!ab.improper_intersects(&cd));
        assert!(!cd.improper_intersects(&ab));
        assert!( ab.intersects(&cd));
        assert!( cd.intersects(&ab));
    }

    #[test]
    fn test_improper_intersect() {
        let a = Point::new(6.0, 6.0);
        let b = Point::new(0.0, 6.0);
        let c = Point::new(1.0, 0.0);
        let d = Point::new(4.0, 6.0);

        let ab = LineSegment::new(&a, &b);
        let cd = LineSegment::new(&c, &d);

        assert!(!ab.proper_intersects(&cd));
        assert!(!cd.proper_intersects(&ab));
        assert!( ab.improper_intersects(&cd));
        assert!( cd.improper_intersects(&ab));
        assert!( ab.intersects(&cd));
        assert!( cd.intersects(&ab));
    }

    #[test]
    fn test_no_intersect() {
        let a = Point::new(6.0, 4.0);
        let b = Point::new(4.0, 4.0);
        let c = Point::new(1.0, 0.0);
        let d = Point::new(4.0, 6.0);

        let ab = LineSegment::new(&a, &b);
        let cd = LineSegment::new(&c, &d);

        assert!(!ab.proper_intersects(&cd));
        assert!(!cd.proper_intersects(&ab));
        assert!(!ab.improper_intersects(&cd));
        assert!(!cd.improper_intersects(&ab));
        assert!(!ab.intersects(&cd));
        assert!(!cd.intersects(&ab));
    }

    #[test]
    fn test_intersect_with_self() {
        let a = Point::new(6.0, 4.0);
        let b = Point::new(4.0, 4.0);
        let ab = LineSegment::new(&a, &b);

        assert!(!ab.proper_intersects(&ab));
        assert!( ab.improper_intersects(&ab));
        assert!( ab.intersects(&ab));
    }

    #[test]
    fn test_reverse() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 2.0);
        let ab = LineSegment::new(&a, &b);
        let ba = ab.reverse();
        assert_eq!(ab.p1, &a);
        assert_eq!(ab.p2, &b);
        assert_eq!(ba.p1, &b);
        assert_eq!(ba.p2, &a);
    }
}