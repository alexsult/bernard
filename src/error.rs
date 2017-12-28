use std::{fmt, error};
use hyper;
use serde_json;
use uuid;

#[derive(Debug)]
pub enum Error {
    Http(String),
    Get(hyper::Error),
    ParseJson(serde_json::Error),
    ParseUuid(uuid::ParseError),
    AsSlice
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref s) => write!(f, "error requesting resource: {}", s),
            Error::Get(ref e) => write!(f, "HTTP Get error {}", e),
            Error::ParseJson(ref e) => write!(f, "error parsing json: {}", e),
            Error::ParseUuid(ref e) => write!(f, "error parsing uuid: {}", e),
            Error::AsSlice => write!(f, "error parsing as slice"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(_) => "error requesting resource",
            Error::Get(ref err) => err.description(),
            Error::ParseJson(ref err) => err.description(),
            Error::ParseUuid(_) => "error parsing uuid",
            Error::AsSlice => "error parsing as slice"
        }
    }
}
