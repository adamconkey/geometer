
use itertools::Itertools;

// TODO make this type-generic?
// TODO handle dimensionality of coordinates
#[derive(Debug)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub index: u32,
}


// TODO I think it might be worth defining an enum here for
// left / collinear / right that resolves to 1 / 0 / -1 if
// possible, so that you could do away with the left and
// collinear tests and instead have a function that determines
// the vertex's relation to two other vertices being one of
// those 3 options. And it can be computed by getting the
// sign of the area. Then it should be easy to define the
// proper intsersection function. Will want to make some
// good unit tests for this functionality


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

    pub fn left(&self, a: &Vertex, b: &Vertex) -> bool {
        triangle_double_area(a, b, self) > 0
    }

    pub fn left_on(&self, a: &Vertex, b: &Vertex) -> bool {
        triangle_double_area(a, b, self) >= 0
    }

    pub fn collinear(&self, a: &Vertex, b: &Vertex) -> bool {
        triangle_double_area(a, b, self) == 0
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
}
