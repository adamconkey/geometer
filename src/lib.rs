
// TODO make this type-generic?
// TODO handle dimensionality of coordinates
pub struct Point {
    pub x: f64,
    pub y: f64,
}


pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {

    pub fn new(a: Point, b: Point, c: Point) -> Triangle {
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
        let a = Point {x: 0.0, y: 0.0};
        let b = Point {x: 3.0, y: 0.0};
        let c = Point {x: 0.0, y: 4.0};
        let triangle = Triangle::new(a, b, c);
        let area = triangle.area();
        assert_eq!(area, 6.0);
    }
}
