
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
