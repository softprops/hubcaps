//! Client errors

use std::io::Error as IoError;
use std::time::Duration;

use http::uri::InvalidUri;
use http::Error as HttpError;
use hyper::Error as HyperError;
use hyper::StatusCode;
use jwt::errors::Error as JWTError;
use serde_json::error::Error as SerdeError;

error_chain! {
    errors {
        #[doc = "Client side error returned for faulty requests"]
        Fault {
            code: StatusCode,
            error: ClientError,
        } {
            display("{}: '{}'", code, error.message)
            description(error.message.as_str())
          }
        #[doc = "Error kind returned when a credential's rate limit has been exhausted. Wait for the reset duration before issuing more requests"]
        RateLimit {
            reset: Duration
        } {
            display("Rate limit exhausted. Will reset in {} seconds", reset.as_secs())
        }
    }
    foreign_links {
        Codec(SerdeError);
        Http(HttpError);
        Hyper(HyperError);
        IO(IoError);
        URI(InvalidUri);
        JWT(JWTError);
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
    use serde_json;
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
            documentation_url: Some(String::from("https://developer.github.com/v3/activity/watching/#set-a-repository-subscription")),
        };
        assert_eq!(serde_json::from_value::<ClientError>(json).unwrap(), expect)
    }
}
