use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]

pub struct Point {
    pub x: i32,
    pub y: i32,
}


impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_serialize_point() {
        let p = Point::new(1, 2);
        let serialized = serde_json::to_string(&p).unwrap();
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, p);
    }

    #[test]
    fn test_serialize_points_vec() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let points = vec![p1, p2];
        let serialized = serde_json::to_string(&points).unwrap();
        let deserialized: Vec<Point> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, points);
    }
}