//! Client errors

use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::StatusCode;
use hyper::error::UriError;
use serde_json::error::Error as SerdeError;

error_chain! {
    errors {
        Fault {
            code: StatusCode,
            error: ClientError,
        } {
            display("{}: '{}'", code, error.message)
            description(error.message.as_str())
          }
    }
    foreign_links {
        Codec(SerdeError);
        Http(HttpError);
        IO(IoError);
        URI(UriError);
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
}

#[cfg(test)]
mod tests {
    use serde_json;
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
                    errors: Some(vec![
                        FieldErr {
                            resource: "Release".to_owned(),
                            code: "custom".to_owned(),
                            field: None,
                            message: Some(
                                "Published releases \
                                                                            must have a valid tag"
                                    .to_owned()
                            ),
                            documentation_url: None,
                        },
                    ]),
                }
            ),
        ]
        {
            assert_eq!(serde_json::from_str::<ClientError>(json).unwrap(), expect);
        }
    }
}
