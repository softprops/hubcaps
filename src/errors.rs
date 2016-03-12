//! Client errors

use std::error::Error as StdError;
use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::status::StatusCode;
use rep::ClientError;
use serde_json::error::Error as SerdeError;

/// enumerated types of client errors
#[derive(Debug)]
pub enum Error {
    Codec(SerdeError),
    Http(HttpError),
    IO(IoError),
    Parse(String),
    Fault {
        code: StatusCode,
        error: ClientError,
    },
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Codec(ref e) => e.description(),
            Error::Http(ref e) => e.description(),
            Error::IO(ref e) => e.description(),
            Error::Parse(ref e) => &e[..],
            Error::Fault { ref error, .. } => &error.message[..],
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Codec(ref e) => Some(e),
            Error::Http(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            _ => None
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        Error::Codec(error)
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
