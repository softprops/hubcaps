//! Statuses interface
extern crate futures;
extern crate serde;
extern crate serde_json;

use hyper::client::connect::Connect;
use users::User;
use {Future, Github};

/// interface for statuses associated with a repository
pub struct Statuses<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> Statuses<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Statuses {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/statuses{}", self.owner, self.repo, more)
    }

    /// creates a new status for a target sha
    pub fn create(&self, sha: &str, status: &StatusOptions) -> Future<Status> {
        self.github
            .post(&self.path(&format!("/{}", sha)), json!(status))
    }

    /// lists all statuses associated with a given git sha
    pub fn list(&self, sha: &str) -> Future<Vec<Status>> {
        self.github.get(&format!(
            "/repos/{}/{}/commits/{}/statuses",
            self.owner, self.repo, sha
        ))
    }

    /// list the combined statuses for a given git sha
    /// fixme: give this a type
    pub fn combined(&self, sha: &str) -> Future<String> {
        self.github.get(&format!(
            "/repos/{}/{}/commits/{}/status",
            self.owner, self.repo, sha
        ))
    }
}

// representations (todo: replace with derive_builder)

#[derive(Debug, Deserialize)]
pub struct Status {
    pub created_at: String,
    pub updated_at: String,
    pub state: State,
    pub target_url: String,
    pub description: String,
    pub id: u64,
    pub url: String,
    pub context: String,
    pub creator: User,
}

#[derive(Debug, Default, Serialize)]
pub struct StatusOptions {
    state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
}

pub struct StatusOptionsBuilder(StatusOptions);

impl StatusOptionsBuilder {
    #[doc(hidden)]
    pub(crate) fn new(state: State) -> Self {
        StatusOptionsBuilder(StatusOptions {
            state,
            ..Default::default()
        })
    }

    pub fn target_url<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.0.target_url = Some(url.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut Self
    where
        D: Into<String>,
    {
        self.0.description = Some(desc.into());
        self
    }

    pub fn context<C>(&mut self, ctx: C) -> &mut Self
    where
        C: Into<String>,
    {
        self.0.context = Some(ctx.into());
        self
    }

    pub fn build(&self) -> StatusOptions {
        StatusOptions::new(
            self.0.state.clone(),
            self.0.target_url.clone(),
            self.0.description.clone(),
            self.0.context.clone(),
        )
    }
}

impl StatusOptions {
    #[doc(hidden)]
    pub fn new<T, D, C>(
        state: State,
        target_url: Option<T>,
        descr: Option<D>,
        context: Option<C>,
    ) -> Self
    where
        T: Into<String>,
        D: Into<String>,
        C: Into<String>,
    {
        StatusOptions {
            state,
            target_url: target_url.map(|t| t.into()),
            description: descr.map(|d| d.into()),
            context: context.map(|c| c.into()),
        }
    }

    pub fn builder(state: State) -> StatusOptionsBuilder {
        StatusOptionsBuilder::new(state)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum State {
    /// pending
    #[serde(rename = "pending")]
    Pending,
    /// success
    #[serde(rename = "success")]
    Success,
    /// error
    #[serde(rename = "error")]
    Error,
    /// failure
    #[serde(rename = "failure")]
    Failure,
}

impl Default for State {
    fn default() -> State {
        State::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::ser::Serialize;
    use serde_json;

    fn test_encoding<E: Serialize>(tests: Vec<(E, &str)>) {
        for test in tests {
            match test {
                (k, v) => assert_eq!(serde_json::to_string(&k).unwrap(), v),
            }
        }
    }

    #[test]
    fn deserialize_status_state() {
        for (json, value) in vec![
            ("\"pending\"", State::Pending),
            ("\"success\"", State::Success),
            ("\"error\"", State::Error),
            ("\"failure\"", State::Failure),
        ] {
            assert_eq!(serde_json::from_str::<State>(json).unwrap(), value)
        }
    }

    #[test]
    fn serialize_status_state() {
        for (json, value) in vec![
            ("\"pending\"", State::Pending),
            ("\"success\"", State::Success),
            ("\"error\"", State::Error),
            ("\"failure\"", State::Failure),
        ] {
            assert_eq!(serde_json::to_string(&value).unwrap(), json)
        }
    }

    #[test]
    fn status_reqs() {
        let tests = vec![
            (
                StatusOptions::builder(State::Pending).build(),
                r#"{"state":"pending"}"#,
            ),
            (
                StatusOptions::builder(State::Success)
                    .target_url("http://acme.com")
                    .build(),
                r#"{"state":"success","target_url":"http://acme.com"}"#,
            ),
            (
                StatusOptions::builder(State::Error)
                    .description("desc")
                    .build(),
                r#"{"state":"error","description":"desc"}"#,
            ),
            (
                StatusOptions::builder(State::Failure)
                    .target_url("http://acme.com")
                    .description("desc")
                    .build(),
                r#"{"state":"failure","target_url":"http://acme.com","description":"desc"}"#,
            ),
        ];
        test_encoding(tests)
    }

}
