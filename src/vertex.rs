
use unique_id::Generator;
use unique_id::sequence::SequenceGenerator;


lazy_static!(
    static ref ID_GENERATOR: SequenceGenerator = SequenceGenerator::default();
);


// TODO make this type-generic?
// TODO handle dimensionality of coordinates
#[derive(Debug)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub index: i64,
}


impl Vertex {
    pub fn new(x: i32, y: i32) -> Vertex {
        Vertex { x: x, y: y, index: ID_GENERATOR.next_id() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let v0 = Vertex::new(1, 2);
        let v1 = Vertex::new(3, 4);
        assert_ne!(v0.index, v1.index);
    }
}
