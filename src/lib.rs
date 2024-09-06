
use itertools::Itertools;

// TODO make this type-generic?
// TODO handle dimensionality of coordinates
#[derive(Debug)]
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


fn triangle_area(a: &Vertex, b: &Vertex, c: &Vertex) -> f64 {
    let t1 = (b.x - a.x) * (c.y - a.y);
    let t2 = (c.x - a.x) * (b.y - a.y);
    0.5 * (t1 - t2)              
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
    
    pub fn area(&self) -> f64 {
        // TODO for now we'll just panic if we access something
        // that doesn't exist, which shouldn't happen if it's
        // a well-defined polygon. So maybe we do some validation
        // to ensure it's valid before even trying computations
        // on it?
        let p = &self.vertices[0];
        let mut area = 0.0;
        for (b, c) in self.vertices.iter().tuple_windows() {
            area += triangle_area(p, b, c);
        }
        area
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
        let mut polygon = Polygon::new();
        polygon.add_vertex(a);
        polygon.add_vertex(b);
        polygon.add_vertex(c);
        let area = polygon.area();
        assert_eq!(area, 6.0);
    }
}
