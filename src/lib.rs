
// TODO make this type-generic?
// TODO handle dimensionality of coordinates
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub index: u32,
}

impl Vertex {
    // TODO it will be nice to not have to reason about
    // indices, can look into making some kind of global
    // integer generator that creates unique indices
    pub fn new(x: f64, y: f64, index: u32) -> Vertex {
        Vertex {
            x: x,
            y: y,
            index: index,
        }
    }
}


pub struct Polygon {
    // TODO
}

impl Polygon {
    pub fn new() -> Polygon {
        // TODO
        Polygon { }
    }

    pub fn area(&self) -> f64 {
        unimplemented!()
    }
}


pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Triangle {
        Triangle { a: a, b: b, c: c }
    }
    
    pub fn area(&self) -> f64 {
        let t1 = (self.b.x - self.a.x) * (self.c.y - self.a.y);
        let t2 = (self.c.x - self.a.x) * (self.b.y - self.a.y);
        0.5 * (t1 - t2)              
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_right_triangle() {
        let a = Vertex::new(0.0, 0.0, 0);
        let b = Vertex::new(3.0, 0.0, 1);
        let c = Vertex::new(0.0, 4.0, 2);
        let triangle = Triangle::new(a, b, c);
        let area = triangle.area();
        assert_eq!(area, 6.0);
    }
}
