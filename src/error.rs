use std::io;


#[derive(Debug)]
pub enum FileError {
    IOError(io::Error),
    ParseError(serde_json::Error),
}

impl From<io::Error> for FileError {
    fn from(value: io::Error) -> Self {
        FileError::IOError(value)
    }
}

impl From<serde_json::Error> for FileError {
    fn from(value: serde_json::Error) -> Self {
        FileError::ParseError(value)
    }
}