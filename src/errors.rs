//! Client errors

use std::error::Error as StdError;
use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::status::StatusCode;
use serde_json::error::Error as SerdeError;

// todo: look into error_chain crate to remove boiler plate

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
            _ => None,
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

// preresentations

#[derive(Debug, Deserialize, PartialEq)]
pub struct FieldErr {
    pub resource: String,
    pub field: Option<String>,
    pub code: String,
    pub message: Option<String>,
    pub documentation_url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClientError {
    pub message: String,
    pub errors: Option<Vec<FieldErr>>,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::{ClientError, FieldErr};
    #[test]
    fn deserialize_client_field_errors() {
        for (json, expect) in vec![// see https://github.com/softprops/hubcaps/issues/31
                                   (r#"{"message": "Validation Failed","errors":
                [{
                    "resource": "Release",
                    "code": "custom",
                    "message": "Published releases must have a valid tag"
                }]}"#,
                                    ClientError {
                                       message: "Validation Failed".to_owned(),
                                       errors: Some(vec![FieldErr {
                                                             resource: "Release".to_owned(),
                                                             code: "custom".to_owned(),
                                                             field: None,
                                                             message: Some("Published releases \
                                                                            must have a valid tag"
                                                                 .to_owned()),
                                                             documentation_url: None,
                                                         }]),
                                   })] {
            assert_eq!(serde_json::from_str::<ClientError>(json).unwrap(), expect);
        }
    }
}
