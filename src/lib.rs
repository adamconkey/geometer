
#[macro_use]
extern crate lazy_static;


pub mod polygon;
pub mod triangle;
pub mod vertex;


// TODO want to implement intersection of line segments. I'm not totally
// sure where to put it though. On some level it could make sense in the
// polygon struct, but I suspect we'll be using that prior to polygon
// construction? Or it will be used for triangulation tests and so forth,
// so maybe for the timebeing define a simple line segment module and
// struct and implement a function in there to compute it. What's
// interesting is triangle area is a pretty fundamental concept to do
// tests on 3-tuples of vertices (which is effectively a triangle), so
// even for things that aren't about triangles (line segment intercects)
// we still use the triangle computations. It points to it really acting
// as a computational unit (taking in refs of vertices) instead of it
// being its own entity (which I think would be if it _owned_ the
// vertex objects themselves)
