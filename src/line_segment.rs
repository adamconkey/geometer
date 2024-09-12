
use crate::triangle::Triangle;
use crate::vertex::Vertex;


pub struct LineSegment<'a> {
    pub a: &'a Vertex,
    pub b: &'a Vertex,
}


pub fn proper_intersection(ls1: &LineSegment, ls2: &LineSegment) -> bool {

    // TODO followed these naming conventions to follow the book text,
    // but having lots of a b c floating around may not be the best
    // idea. Should probably look at other libraries to see how they
    // handle naming conventions for points in geometric entities
    
    let a = ls1.a;
    let b = ls1.b;
    let c = ls2.a;
    let d = ls2.b;

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


// TODO need some unit tests for this
