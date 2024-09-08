
use itertools::Itertools;

// TODO make this type-generic?
// TODO handle dimensionality of coordinates
#[derive(Debug)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub index: u32,
}


// TODO not sure yet if this is useful, will want to implement
// the intersection function and see if it's worthwhile.
// If the enum is unnecessary then can just have the function
// return 0, -1, 1, and use those values directly

#[derive(Debug, PartialEq)]
pub enum PointLineRelation {
    Collinear = 0,
    Left = 1,
    Right = -1,
}


impl PointLineRelation {
    pub fn from_i32(value: i32) -> PointLineRelation {
        match value {    
            -1 => PointLineRelation::Right,
            0 => PointLineRelation::Collinear,
            1 => PointLineRelation::Left,
            _ => panic!("Undefined for i32 not in [-1, 0, 1]"),
        }
    }
}


// TODO want some better unit tests for the triangle area,
// and also more general polygon areas.


// TODO not sure what to do with this function yet, may end up
// having a triangle type that we'll put this on
fn triangle_double_area(a: &Vertex, b: &Vertex, c: &Vertex) -> i32 {
    let t1 = (b.x - a.x) * (c.y - a.y);
    let t2 = (c.x - a.x) * (b.y - a.y);
    t1 - t2
}


impl Vertex {
    // TODO it will be nice to not have to reason about
    // indices, can look into making some kind of global
    // integer generator that creates unique indices
    pub fn new(x: i32, y: i32, index: u32) -> Vertex {
        Vertex {
            x: x,
            y: y,
            index: index,
        }
    }

    pub fn relation_to_line(&self, a: &Vertex, b: &Vertex) -> PointLineRelation {
        PointLineRelation::from_i32(triangle_double_area(a, b, self).signum())
    }
}




pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

impl Polygon {
    // TODO want vec-based new func
    
    pub fn new() -> Polygon {
        Polygon { vertices: Vec::new() }
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        self.vertices.push(v);
    }
    
    pub fn double_area(&self) -> i32 {
        // TODO for now we'll just panic if we access something
        // that doesn't exist, which shouldn't happen if it's
        // a well-defined polygon. So maybe we do some validation
        // to ensure it's valid before even trying computations
        // on it?

        // The first tuple will include the anchor, but that area
        // ends up being zero since p2 will trivially be collinear
        // with anchor-p1 and thus doesn't affect the compuation
        let anchor = &self.vertices[0];
        let mut area = 0;
        for (p1, p2) in self.vertices.iter().tuple_windows() {
            area += triangle_double_area(anchor, p1, p2);
        }
        area
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_right_triangle() {
        let a = Vertex::new(0, 0, 0);
        let b = Vertex::new(3, 0, 1);
        let c = Vertex::new(0, 4, 2);
        let mut polygon = Polygon::new();
        polygon.add_vertex(a);
        polygon.add_vertex(b);
        polygon.add_vertex(c);
        let double_area = polygon.double_area();
        assert_eq!(double_area, 12);
    }

    #[test]
    fn test_point_line_relations_triangle() {
        let a = Vertex::new(0, 0, 0);
        let b = Vertex::new(4, 3, 1);
        let c = Vertex::new(1, 3, 2);
        let a_to_bc = a.relation_to_line(&b, &c);
        let a_to_cb = a.relation_to_line(&c, &b);
        let b_to_ac = b.relation_to_line(&a, &c);
        let b_to_ca = b.relation_to_line(&c, &a);
        let c_to_ab = c.relation_to_line(&a, &b);
        let c_to_ba = c.relation_to_line(&b, &a);
        assert_eq!(a_to_bc, PointLineRelation::Left);
        assert_eq!(a_to_cb, PointLineRelation::Right);
        assert_eq!(b_to_ac, PointLineRelation::Right);
        assert_eq!(b_to_ca, PointLineRelation::Left);
        assert_eq!(c_to_ab, PointLineRelation::Left);
        assert_eq!(c_to_ba, PointLineRelation::Right);
    }

    #[test]
    fn test_collinear() {
        let a = Vertex::new(0, 0, 0);
        let b = Vertex::new(4, 3, 1);
        let c = Vertex::new(8, 6, 2);
        let relations = vec![
            a.relation_to_line(&b, &c),
            a.relation_to_line(&c, &b),
            b.relation_to_line(&a, &c),
            b.relation_to_line(&c, &a),
            c.relation_to_line(&a, &b),
            c.relation_to_line(&b, &a),
        ];
        for relation in relations {
            assert_eq!(relation, PointLineRelation::Collinear);
        }
    }
}
