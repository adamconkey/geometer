use unique_id::Generator;
use unique_id::sequence::SequenceGenerator;

// TODO will likely want to rethink this structure, will there be a
// triangle primitive? If so then obviously the area can go on that,
// and on some level it makese sense to also  consider point line
// (segment) relations in terms of triangles.
use crate::PointLineRelation;
use crate::triangle_double_area;


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
        Vertex {
            x: x,
            y: y,
            index: ID_GENERATOR.next_id(),
        }
    }

    pub fn relation_to_line(&self, a: &Vertex, b: &Vertex) -> PointLineRelation {
        PointLineRelation::from_i32(triangle_double_area(a, b, self).signum())
    }
}
