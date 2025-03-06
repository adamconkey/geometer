use clap::{Parser, ValueEnum};
use std::fs;
use std::{ffi::OsStr, path::PathBuf};
use serde_json::{Map, Value};
use walkdir::WalkDir;

use geometer::error::FileError;


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Action {
    Add,
    Remove,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FieldType {
    Number, 
    Vec,
}



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    field: String,

    #[arg(short, long)]
    action: Action,

    #[arg(long, default_value = "number")]
    field_type: FieldType,
}


fn main() -> Result<(), FileError> {
    let args = Args::parse();
    

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("polygons");
    let paths = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file())
        .filter(|p| p.extension() == Some(OsStr::new("json")))
        .filter(|p| p.with_extension("").extension() == Some(OsStr::new("meta")));
    
    for path in paths {
        let metadata_str: String = fs::read_to_string(&path)?;
        let mut metadata: Map<String, Value> = serde_json::from_str(&metadata_str)?;

        // TODO should be able to read from args from here to do what you want

        metadata.remove("num_vertices");

        // let metadata_str = serde_json::to_string_pretty(&metadata)?;
        // fs::write(path, metadata_str)?;

        println!("META: {:?}", metadata);
    }

    Ok(())
}