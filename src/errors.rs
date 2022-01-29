//! Client errors
#[cfg(feature = "jwt")]
use crate::jwt::errors::Error as JWTError;
use http::StatusCode;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::error::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::result::Result as StdResult;
use std::time::Duration;
use url::ParseError;

/// A standard result type capturing common errors for all GitHub operations
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Client side error returned for faulty requests
    Fault {
        code: StatusCode,
        error: ClientError,
    },
    /// Error kind returned when a credential's rate limit has been exhausted. Wait for the reset duration before issuing more requests
    RateLimit { reset: Duration },
    /// Serialization related errors
    Codec(SerdeError),
    /// HTTP client errors
    Reqwest(ReqwestError),
    /// Url format errors
    Url(ParseError),
    /// Network errors
    IO(IoError),

    #[cfg(feature = "jwt")]
    /// JWT validation errors
    JWT(JWTError),
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::Codec(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::Url(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IO(err)
    }
}

#[cfg(feature = "jwt")]
impl From<JWTError> for Error {
    fn from(err: JWTError) -> Self {
        Error::JWT(err)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Codec(err) => Some(err),
            Error::Reqwest(err) => Some(err),
            Error::Url(err) => Some(err),
            Error::IO(err) => Some(err),
            #[cfg(feature = "jwt")]
            Error::JWT(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Fault { code, error } => write!(f, "{}: {}", code, error.message),
            Error::RateLimit { reset } => write!(
                f,
                "Rate limit exhausted. Will reset in {} seconds",
                reset.as_secs()
            ),
            Error::Codec(err) => write!(f, "{}", err),
            Error::Reqwest(err) => write!(f, "{}", err),
            Error::Url(err) => write!(f, "{}", err),
            Error::IO(err) => write!(f, "{}", err),
            #[cfg(feature = "jwt")]
            Error::JWT(err) => write!(f, "{}", err),
        }
    }
}

// representations

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
    pub documentation_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{ClientError, FieldErr};

    #[test]
    fn deserialize_client_field_errors() {
        for (json, expect) in vec![
            // see https://github.com/softprops/hubcaps/issues/31
            (
                r#"{"message": "Validation Failed","errors":
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
                        message: Some(
                            "Published releases \
                             must have a valid tag"
                                .to_owned(),
                        ),
                        documentation_url: None,
                    }]),
                    documentation_url: None,
                },
            ),
        ] {
            assert_eq!(serde_json::from_str::<ClientError>(json).unwrap(), expect);
        }
    }

    #[test]
    fn deserialize_client_top_level_documentation_url() {
        let json = serde_json::json!({
            "message": "Not Found",
            "documentation_url": "https://developer.github.com/v3/activity/watching/#set-a-repository-subscription"
        });
        let expect = ClientError {
            message: String::from("Not Found"),
            errors: None,
            documentation_url: Some(String::from(
                "https://developer.github.com/v3/activity/watching/#set-a-repository-subscription",
            )),
        };
        assert_eq!(serde_json::from_value::<ClientError>(json).unwrap(), expect)
    }
}
