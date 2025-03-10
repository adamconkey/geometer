use crate::{
    point::Point,
    triangle::Triangle,
    vector::Vector,
    vertex::Vertex,
};


#[derive(Clone, Debug, PartialEq)]
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

    pub fn length(&self) -> f64 {
        Vector::from(self).magnitude()
    }

    pub fn angle_to_point(&self, p: &Point) -> f64 {
        // TODO this is a specific interpretation of "angle to point"
        // which is between two vectors p1 -> p2 and p2 -> p. Not
        // sure it's the best named function but it's nice to have
        // the function as it makes sorting vecs by the value very
        // compact. Can consider moving to another struct or renaming
        let p1_to_p2 = Vector::from(self);
        let p2_to_p = Vector::from(&LineSegment::new(self.p2, p));
        let cos_theta = p1_to_p2.dot(&p2_to_p) / (p1_to_p2.magnitude() * p2_to_p.magnitude());
        cos_theta.acos()
    }

    pub fn distance_to_point(&self, p: &Point) -> f64 {
        // https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
        let p1_to_p2 = Vector::from(self);
        let numerator = (
            p1_to_p2.y * p.x - 
            p1_to_p2.x * p.y + 
            self.p2.x * self.p1.y - 
            self.p2.y * self.p1.x
        ).abs(); 
        numerator / p1_to_p2.magnitude()
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