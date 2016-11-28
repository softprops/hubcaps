//! Rust representations of Github API data structures

use super::{SortDirection, Error, State as StdState};
use super::issues::Sort as IssueSort;
use super::search::SearchIssuesSort;
use super::repositories::{Sort as RepoSort, Affiliation, Type as RepoType,
                          Visibility as RepoVisibility, OrgRepoType};
use std::collections::HashMap;
use std::hash::Hash;
use std::option::Option;
use url::form_urlencoded;

use super::url;
extern crate serializable_enum;
extern crate serde;
extern crate serde_json;

include!(concat!(env!("OUT_DIR"), "/rep.rs"));

serializable_enum! {
    /// representation of deployment and commit status states
    #[derive(Clone, Debug, PartialEq)]
    pub enum StatusState {
        /// pending
        Pending,
        /// success
        Success,
        /// error
        Error,
        /// failure
        Failure,
    }
    StatusStateVisitor
}

impl_as_ref_from_str! {
    StatusState {
        Pending => "pending",
        Success => "success",
        Error => "error",
        Failure => "failure",
    }
    Error::Parse
}

impl Default for StatusState {
    fn default() -> StatusState {
        StatusState::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::super::State as StdState;
    use serde::ser::Serialize;
    use std::collections::{HashMap, BTreeMap};
    use super::*;
    use super::serde_json;

    fn test_encoding<E: Serialize>(tests: Vec<(E, &str)>) {
        for test in tests {
            match test {
                (k, v) => assert_eq!(serde_json::to_string(&k).unwrap(), v),
            }
        }
    }

    #[test]
    fn gist_reqs() {
        let mut files = HashMap::new();
        files.insert("foo", "bar");
        let tests = vec![(GistOptions::new(None as Option<String>, true, files.clone()),
                  r#"{"public":true,"files":{"foo":{"content":"bar"}}}"#),
                 (GistOptions::new(Some("desc"), true, files.clone()),
                  r#"{"description":"desc","public":true,"files":{"foo":{"content":"bar"}}}"#)];
        test_encoding(tests);
    }

    #[test]
    fn deserialize_client_field_errors() {
        for (json, expect) in vec![
            // see https://github.com/softprops/hubcaps/issues/31
            (
                r#"{"message": "Validation Failed","errors": [{"resource": "Release","code": "custom","message": "Published releases must have a valid tag"}]}"#,
                ClientError {
                    message:"Validation Failed".to_owned(),
                    errors: Some(vec![
                        FieldErr {
                            resource: "Release".to_owned(),
                            code:"custom".to_owned(),
                            field: None,
                            message: Some("Published releases must have a valid tag".to_owned()),
                            documentation_url: None
                        }
                     ])
                }
             )
        ] {
            assert_eq!(serde_json::from_str::<ClientError>(json).unwrap(), expect);
        }
    }

    #[test]
    fn deserialize_status_state() {
        for (json, expect) in vec![("\"pending\"", StatusState::Pending),
                                   ("\"success\"", StatusState::Success),
                                   ("\"error\"", StatusState::Error),
                                   ("\"failure\"", StatusState::Failure)] {
            assert_eq!(serde_json::from_str::<StatusState>(json).unwrap(), expect)
        }
    }

    #[test]
    fn deployment_reqs() {
        let mut payload = BTreeMap::new();
        payload.insert("user", "atmos");
        payload.insert("room_id", "123456");
        let tests = vec![
            (
                DeploymentOptions::builder("test").build(),
                r#"{"ref":"test"}"#
            ),
            (
                DeploymentOptions::builder("test").task("launchit").build(),
                r#"{"ref":"test","task":"launchit"}"#
            ),
            (
                DeploymentOptions::builder("topic-branch").description("description").payload(payload).build(),
                r#"{"ref":"topic-branch","payload":"{\"room_id\":\"123456\",\"user\":\"atmos\"}","description":"description"}"#
            )
        ];
        test_encoding(tests)
    }

    #[test]
    fn deployment_status_reqs() {
        let tests = vec![
            (
                DeploymentStatusOptions::builder(StatusState::Pending).build(),
                r#"{"state":"pending"}"#
            ),
            (
                DeploymentStatusOptions::builder(StatusState::Pending).target_url("http://host.com").build(),
                r#"{"state":"pending","target_url":"http://host.com"}"#
            ),
            (
                DeploymentStatusOptions::builder(StatusState::Pending).target_url("http://host.com").description("desc").build(),
                r#"{"state":"pending","target_url":"http://host.com","description":"desc"}"#
            ),
        ];
        test_encoding(tests)
    }

    #[test]
    fn gist_req() {
        let mut files = HashMap::new();
        files.insert("test", "foo");
        let tests = vec![(GistOptions::builder(files.clone()).build(),
                  r#"{"files":{"test":{"content":"foo"}}}"#),
                 (GistOptions::builder(files.clone()).description("desc").build(),
                  r#"{"description":"desc","files":{"test":{"content":"foo"}}}"#),
                 (GistOptions::builder(files.clone()).description("desc").public(false).build(),
                  r#"{"description":"desc","public":false,"files":{"test":{"content":"foo"}}}"#)];
        test_encoding(tests)
    }

    #[test]
    fn pullreq_edits() {
        let tests = vec![(PullEditOptions::builder().title("test").build(), r#"{"title":"test"}"#),
                         (PullEditOptions::builder().title("test").body("desc").build(),
                          r#"{"title":"test","body":"desc"}"#),
                         (PullEditOptions::builder().state("closed").build(),
                          r#"{"state":"closed"}"#)];
        test_encoding(tests)
    }

    #[test]
    fn status_reqs() {
        let tests = vec![(StatusOptions::builder(StatusState::Pending).build(),
                  r#"{"state":"pending"}"#),
                 (StatusOptions::builder(StatusState::Success)
                      .target_url("http://acme.com")
                      .build(),
                  r#"{"state":"success","target_url":"http://acme.com"}"#),
                 (StatusOptions::builder(StatusState::Error).description("desc").build(),
                  r#"{"state":"error","description":"desc"}"#),
                 (StatusOptions::builder(StatusState::Failure)
                      .target_url("http://acme.com")
                      .description("desc")
                      .build(),
                  r#"{"state":"failure","target_url":"http://acme.com","description":"desc"}"#)];
        test_encoding(tests)
    }

    #[test]
    fn issue_list_reqs() {
        fn test_serialize(tests: Vec<(IssueListOptions, Option<String>)>) {
            for test in tests {
                match test {
                    (k, v) => assert_eq!(k.serialize(), v),
                }
            }
        }
        let tests = vec![
            (
                IssueListOptions::builder().build(),
                None
            ),
            (
                IssueListOptions::builder().state(StdState::Closed).build(),
                Some("state=closed".to_owned())
             ),
            (
                IssueListOptions::builder().labels(vec!["foo", "bar"]).build(),
                Some("labels=foo%2Cbar".to_owned())
            ),
        ];
        test_serialize(tests)
    }

    #[test]
    fn pull_list_reqs() {
        fn test_serialize(tests: Vec<(PullListOptions, Option<String>)>) {
            for test in tests {
                match test {
                    (k, v) => assert_eq!(k.serialize(), v),
                }
            }
        }
        let tests = vec![(PullListOptions::builder().build(), None),
                         (PullListOptions::builder().state(StdState::Closed).build(),
                          Some("state=closed".to_owned()))];
        test_serialize(tests)
    }
}
