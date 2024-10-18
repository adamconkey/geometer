use std::str::FromStr;
use unique_id::Generator;
use unique_id::string::StringGenerator;

use crate::{
    line_segment::LineSegment,
    triangle::Triangle,
};


lazy_static!(
    static ref ID_GENERATOR: StringGenerator = StringGenerator::default();
);


// TODO make this type-generic?
// TODO handle dimensionality of coordinates
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub id: String,
}


#[derive(Debug, PartialEq)]
pub struct ParseVertexError;


impl FromStr for Vertex {
    type Err = ParseVertexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let x = split
            .next()
            .ok_or(ParseVertexError)?
            .parse::<i32>()
            .map_err(|_| ParseVertexError)?;
        let y = split
            .next()
            .ok_or(ParseVertexError)?
            .parse::<i32>()
            .map_err(|_| ParseVertexError)?;
        let id = split.next().ok_or(ParseVertexError)?
            .to_string();

        Ok(Vertex::new_with_id(x, y, id))        
    }
    
}


impl Vertex {
    pub fn new(x: i32, y: i32) -> Vertex {
        Vertex { x, y, id: ID_GENERATOR.next_id() }
    }

    pub fn new_with_id(x: i32, y: i32, id: String) -> Vertex {
        Vertex { x, y, id }
    }

    pub fn between(&self, a: &Vertex, b: &Vertex) -> bool {
        if !Triangle::new(a, b, self).has_collinear_points() {
            return false;
        }

        let (e1, e2, check) = match LineSegment::new(&a, &b).is_vertical() {
            true  => (a.y, b.y, self.y),
            false => (a.x, b.x, self.x),
        };
        
        (e1..e2).contains(&check) || (e2..e1).contains(&check)
    }

    pub fn left(&self, ab: &LineSegment) -> bool {
        Triangle::new(ab.v1, ab.v2, self).area_sign() > 0
    }

    pub fn left_on(&self, ab: &LineSegment) -> bool {
        Triangle::new(ab.v1, ab.v2, self).area_sign() >= 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let v0 = Vertex::new(1, 2);
        let v1 = Vertex::new(3, 4);
        assert_ne!(v0.id, v1.id);
    }

    #[test]
    fn test_from_str() {
        let expected = Ok(Vertex::new_with_id(1, 2, String::from("a")));
        assert_eq!(Vertex::from_str("1 2 a"), expected);
    }
    

    // TODO might be nice to add custom macro for between asserts,
    // not sure how difficult it is to write macros at this stage
    #[test]
    fn test_between() {
        let v0 = Vertex::new(0, 0);
        let v1 = Vertex::new(1, 1);
        let v2 = Vertex::new(2, 2);

        assert!( v1.between(&v0, &v2));
        assert!( v1.between(&v2, &v1));
        assert!(!v0.between(&v1, &v2));
        assert!(!v0.between(&v2, &v1));
        assert!(!v2.between(&v0, &v1));
        assert!(!v2.between(&v1, &v0));
    }

    #[test]
    fn test_between_vertical() {
        let v0 = Vertex::new(0, 0);
        let v1 = Vertex::new(0, 1);
        let v2 = Vertex::new(0, 2);

        assert!( v1.between(&v0, &v2));
        assert!( v1.between(&v2, &v1));
        assert!(!v0.between(&v1, &v2));
        assert!(!v0.between(&v2, &v1));
        assert!(!v2.between(&v0, &v1));
        assert!(!v2.between(&v1, &v0));
    }
}
