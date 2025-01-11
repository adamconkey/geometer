use std::fs;
use std::path::PathBuf;

use computational_geometry::polygon::Polygon;

fn main() {
    let mut dataset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dataset_path.push("resources/test/interesting_polygon_archive");

    let mut orig_path = dataset_path.clone();
    orig_path.push("original");

    for path in fs::read_dir(orig_path).unwrap() {
        let src_json_path = path.unwrap().path();
        let mut dest_json_path = dataset_path.clone();
        dest_json_path.push(src_json_path.file_name().unwrap());

        let mut polygon = Polygon::from_json(src_json_path);
    
        // TODO I'm thinking of adding the ability to compute the
        // bounding box of the polygon, which should be quite easy
        // to implement. Then you could either rotate around the
        // center of the bounding box, or if that produces weird
        // coordinates (I'm not sure yet), then you could rotate
        // PI around the origin as I'm doing here, but translate
        // by essentially the vector between the center of the 
        // new bounding box after rotation and the center of the
        // old bounding box. I think if you rounded those center
        // coordinates it would work out to nice even coordinates. 
        polygon.rotate_about_origin(std::f64::consts::PI);
        polygon.round_coordinates();
        polygon.to_json(dest_json_path);
    }
}