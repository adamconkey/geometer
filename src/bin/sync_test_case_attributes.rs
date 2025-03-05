use std::fs;
use std::{ffi::OsStr, path::PathBuf};
use struct_field_names_as_array::FieldNamesAsArray;
use walkdir::WalkDir;

use geometer::error::FileError;
use geometer::polygon::PolygonMetadata;




fn main() -> Result<(), FileError> {

    // TODO this works for reading off fields as strings, want to take that
    // and run through each file, delete if not present and add if present.
    // Will potentially need to introspect on type though to determine default
    // value for files to parse correctly, and to effectively be a no-op in
    // tests. Need to see what kind of type introspection might exist for Rust
    println!("FIELDS: {:?}", PolygonMetadata::FIELD_NAMES_AS_ARRAY);

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("polygons");
    let paths = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file())
        .filter(|p| p.extension() == Some(OsStr::new("json")))
        .filter(|p| p.with_extension("").extension() == Some(OsStr::new("meta")));
    
    
    Ok(())
}