use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::status::StatusCode;
use rustc_serialize::json::DecoderError;
use rep::ClientError;

/// enumerated types of client errors
#[derive(Debug)]
pub enum Error {
    Decoding(DecoderError),
    Http(HttpError),
    IO(IoError),
    Fault {
        code: StatusCode,
        error: ClientError
    },
}

impl From<DecoderError> for Error {
    fn from(error: DecoderError) -> Error {
        Error::Decoding(error)
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
