use std::fmt;
use std::fmt::Formatter;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FileError(String),
    ParseError(String),
    GenerateError(String),
    ZipError(String),
    CommandError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Error::FileError(e) => write!(f, "{:?}", e),
            Error::ParseError(e) => write!(f, "{:?}", e),
            Error::GenerateError(e) => write!(f, "{:?}", e),
            Error::ZipError(e) => write!(f, "{:?}", e),
            Error::CommandError(e) => write!(f, "{:?}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        return Error::FileError(format!("file error => {:?}", e));
    }
}
