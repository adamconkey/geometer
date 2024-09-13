
use crate::triangle::Triangle;
use crate::vertex::Vertex;


pub struct LineSegment<'a> {
    pub v1: &'a Vertex,
    pub v2: &'a Vertex,
}


impl<'a> LineSegment<'a> {
    pub fn new(v1: &'a Vertex, v2: &'a Vertex) -> Self {
        LineSegment { v1: v1, v2: v2 }
    }
}


pub fn proper_intersection(ab: &LineSegment, cd: &LineSegment) -> bool {    
    let a = ab.v1;
    let b = ab.v2;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_intersection() {
        let a = Vertex::new(6, 4);
        let b = Vertex::new(0, 4);
        let c = Vertex::new(1, 0);
        let d = Vertex::new(4, 6);

        let ab = LineSegment::new(&a, &b);
        let cd = LineSegment::new(&c, &d);

        assert!(proper_intersection(&ab, &cd));
    }

    #[test]
    fn test_proper_intersection_no_intersect() {
        let a = Vertex::new(6, 4);
        let b = Vertex::new(4, 4);
        let c = Vertex::new(1, 0);
        let d = Vertex::new(4, 6);

        let ab = LineSegment::new(&a, &b);
        let cd = LineSegment::new(&c, &d);

        assert!(!proper_intersection(&ab, &cd));
    }

    // TODO need more tests, can check proper func on improper
    // scenario to show it fails, and if vertices coincide. May
    // Want to implement other functions in that section so it's
    // clear what other nominal test cases there might be
   
}
