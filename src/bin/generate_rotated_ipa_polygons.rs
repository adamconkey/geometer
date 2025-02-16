use std::fs;
use std::path::PathBuf;

use geometer::error::FileError;
use geometer::polygon::Polygon;

fn main() -> Result<(), FileError> {
    let mut dataset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dataset_path.push("polygons/interesting_polygon_archive");

    let mut orig_path = dataset_path.clone();
    orig_path.push("original");

    for path in fs::read_dir(orig_path)? {
        let mut dest_json_path = dataset_path.clone();
        let src_json_path = path?.path();
        match src_json_path.file_name() {
            Some(filename) => dest_json_path.push(filename),
            _ => continue,
        }

        let mut polygon = Polygon::from_json(&src_json_path)?;
        
        // Get a (rounded) bounding box center for translation vector
        let orig_bb = polygon.bounding_box();
        let mut orig_bb_center = orig_bb.center();
        orig_bb_center.round();
    
        polygon.rotate_about_origin(std::f64::consts::PI);
        polygon.round_coordinates();
        
        // Get a new (rounded) bounding box center to compute translation
        let new_bb = polygon.bounding_box();
        let mut new_bb_center = new_bb.center();
        new_bb_center.round();

        let x = orig_bb_center.x - new_bb_center.x;
        let y = orig_bb_center.y - new_bb_center.y;
        polygon.translate(x, y);

        let _ = polygon.to_json(dest_json_path);
    }

    Ok(())
}