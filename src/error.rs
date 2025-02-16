use std::io;


#[derive(Debug)]
pub enum FileError {
    IO(io::Error),
    Parse(serde_json::Error),
}

impl From<io::Error> for FileError {
    fn from(value: io::Error) -> Self {
        FileError::IO(value)
    }
}

impl From<serde_json::Error> for FileError {
    fn from(value: serde_json::Error) -> Self {
        FileError::Parse(value)
    }
}