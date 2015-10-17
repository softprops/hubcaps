use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::status::StatusCode;

/// enumerated types of client errors
#[derive(Debug)]
pub enum Error {
    Http(HttpError),
    Io(IoError),
    Fault { code: StatusCode, body: String }
}

impl From<HttpError> for Error {
    fn from(error: HttpError) -> Error {
        Error::Http(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::Io(error)
    }
}
