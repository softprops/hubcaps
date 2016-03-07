//! Client errors

use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::status::StatusCode;
use rep::ClientError;
use serde_json::error::{Error as SerdeError};

/// enumerated types of client errors
#[derive(Debug)]
pub enum Error {
    Serialize(SerdeError),
    Http(HttpError),
    IO(IoError),
    Parse(String),
    Fault {
        code: StatusCode,
        error: ClientError,
    },
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        Error::Serialize(error)
    }
}

impl From<HttpError> for Error {
    fn from(error: HttpError) -> Error {
        Error::Http(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::IO(error)
    }
}
