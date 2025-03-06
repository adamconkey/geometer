use clap::{Parser, ValueEnum};
use std::fs;
use std::{ffi::OsStr, path::PathBuf};
use serde_json::{json, Map, Value};
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
    Array,
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    field: String,

    #[arg(short, long)]
    action: Action,

    #[arg(short = 't', long, default_value = "number")]
    field_type: FieldType,

    #[arg(short, long, default_value = "false")]
    dry_run: bool,
}


fn add_field(metadata: &mut Map<String, Value>, field: &str, field_type: &FieldType) {
    let value = match field_type {
        FieldType::Array => json!([]),
        FieldType::Number => json!(0),
    };

    metadata.insert(field.to_string(), value);
}

fn remove_field(metadata: &mut Map<String, Value>, field: &str) {
    metadata.remove(field);
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

        match args.action {
            Action::Add => add_field(&mut metadata, &args.field, &args.field_type),
            Action::Remove => remove_field(&mut metadata, &args.field),
        };

        if args.dry_run {
            println!("{:?}", metadata);
        } else {
            let metadata_str = serde_json::to_string_pretty(&metadata)?;
            fs::write(path, metadata_str)?;
        }
    }
    
    Ok(())
}