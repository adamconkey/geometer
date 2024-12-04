use crate::{
    point::Point,
    triangle::Triangle,
    vertex::Vertex,
};


#[derive(Debug, Eq, PartialEq)]
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
        
        let ab_splits_cd = abc.area_sign() * abd.area_sign() < 0;
        let cd_splits_ab = cda.area_sign() * cdb.area_sign() < 0;
                
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

    pub fn intersection_with_horizontal(&self, y: i32) -> Option<i32> {
        // If this line segment is entirely above or entirely below the
        // horizontal, there is no intersection
        if (self.p1.y > y && self.p2.y > y) || (self.p1.y < y && self.p2.y < y) {
            return None
        } 
        
        // TODO this can likely be simplified, just doing rote replacement for now
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;
        let x3 = x1;
        let y3 = y;
        let x4 = x2;
        let y4 = y;

        let t1 = x1 * y2 - y1 * x2;
        let t2 = x3 - x4;
        let t3 = x1 - x2;
        let t4 = x3 * y4 - y3 - x4;
        let t5 = y3 - y4;
        let t6 = y1 - y2;
        let denom = t3 * t5 - t6 * t2;

        Some((t1 * t2 - t3 * t4) / denom)

        // TODO definitely need to unit test this if it's kept around
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_proper_intersect() {
        let a = Point::new(6, 4);
        let b = Point::new(0, 4);
        let c = Point::new(1, 0);
        let d = Point::new(4, 6);

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
        let a = Point::new(6, 6);
        let b = Point::new(0, 6);
        let c = Point::new(1, 0);
        let d = Point::new(4, 6);

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
        let a = Point::new(6, 4);
        let b = Point::new(4, 4);
        let c = Point::new(1, 0);
        let d = Point::new(4, 6);

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
        let a = Point::new(6, 4);
        let b = Point::new(4, 4);
        let ab = LineSegment::new(&a, &b);

        assert!(!ab.proper_intersects(&ab));
        assert!( ab.improper_intersects(&ab));
        assert!( ab.intersects(&ab));
    }

    #[test]
    fn test_reverse() {
        let a = Point::new(0, 0);
        let b = Point::new(1, 2);
        let ab = LineSegment::new(&a, &b);
        let ba = ab.reverse();
        assert_eq!(ab.p1, &a);
        assert_eq!(ab.p2, &b);
        assert_eq!(ba.p1, &b);
        assert_eq!(ba.p2, &a);
    }
}