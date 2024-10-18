use crate::{
    triangle::Triangle,
    vertex::Vertex,
};


#[derive(Debug, Eq, PartialEq)]
pub struct LineSegment<'a> {
    pub v1: &'a Vertex,
    pub v2: &'a Vertex,
}


impl<'a> LineSegment<'a> {
    pub fn new(v1: &'a Vertex, v2: &'a Vertex) -> Self {
        LineSegment { v1: v1, v2: v2 }
    }

    pub fn reverse(&self) -> LineSegment {
        LineSegment::new(self.v2, self.v1)
    }
    
    pub fn is_vertical(&self) -> bool {
        self.v1.x == self.v2.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.v1.y == self.v2.y
    }

    pub fn proper_intersects(&self, cd: &LineSegment) -> bool {    
        let a = self.v1;
        let b = self.v2;
        let c = cd.v1;
        let d = cd.v2;
    
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
        let a = self.v1;
        let b = self.v2;
        let c = cd.v1;
        let d = cd.v2;
    
        c.between(a, b) || d.between(a, b) || a.between(c, d) || b.between(c, d)
    }
    
    pub fn intersects(&self, cd: &LineSegment) -> bool {
        self.proper_intersects(cd) || self.improper_intersects(cd)
    }

    pub fn connected_to(&self, cd: &LineSegment) -> bool {
        self.incident_to(cd.v1) || self.incident_to(cd.v2)
    }

    pub fn incident_to(&self, v: &Vertex) -> bool {
        self.v1 == v || self.v2 == v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_intersect() {
        let a = Vertex::new(6, 4);
        let b = Vertex::new(0, 4);
        let c = Vertex::new(1, 0);
        let d = Vertex::new(4, 6);

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
        let a = Vertex::new(6, 6);
        let b = Vertex::new(0, 6);
        let c = Vertex::new(1, 0);
        let d = Vertex::new(4, 6);

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
        let a = Vertex::new(6, 4);
        let b = Vertex::new(4, 4);
        let c = Vertex::new(1, 0);
        let d = Vertex::new(4, 6);

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
        let a = Vertex::new(6, 4);
        let b = Vertex::new(4, 4);
        let ab = LineSegment::new(&a, &b);

        assert!(!ab.proper_intersects(&ab));
        assert!( ab.improper_intersects(&ab));
        assert!( ab.intersects(&ab));
    }

    #[test]
    fn test_reverse() {
        let a = Vertex::new(0, 0);
        let b = Vertex::new(1, 2);
        let ab = LineSegment::new(&a, &b);
        let ba = ab.reverse();
        assert_eq!(ab.v1, &a);
        assert_eq!(ab.v2, &b);
        assert_eq!(ba.v1, &b);
        assert_eq!(ba.v2, &a);
    }
}
