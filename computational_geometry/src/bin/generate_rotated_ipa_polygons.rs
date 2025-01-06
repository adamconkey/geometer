use std::path::PathBuf;


fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test/interesting_polygon_archive");

    let mut orig_path = path.clone();
    orig_path.push("original");

    // TODO want to move all of the current IPA polygons to a
    // subfolder 'original' and then as a starting point, just
    // read them all in and write the polygons to file in the 
    // 'path' directory. That's the basic structure, then just
    // need to decide how to rotate them all. They'll all be
    // rotated the same amount PI, so it's just the origin that
    // matters. Could potentially use the centroid from the
    // dataset.
}