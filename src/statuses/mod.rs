//! Statuses interface
extern crate serde_json;
extern crate serde;

use self::super::{Github, Result};
use users::User;

/// interface for statuses assocaited with a repository
pub struct Statuses<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Statuses<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Statuses<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Statuses {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/statuses{}", self.owner, self.repo, more)
    }

    /// creates a new status for a target sha
    pub fn create(&self, sha: &str, status: &StatusOptions) -> Result<Status> {
        let data = try!(serde_json::to_string(&status));
        self.github.post::<Status>(&self.path(&format!("/{}", sha)), data.as_bytes())
    }

    /// lists all statuses associated with a given git sha
    pub fn list(&self, sha: &str) -> Result<Vec<Status>> {
        self.github.get::<Vec<Status>>(&format!("/repos/{}/{}/commits/{}/statuses",
                                                self.owner,
                                                self.repo,
                                                sha))
    }

    /// list the combined statuses for a given git sha
    pub fn combined(&self, sha: &str) -> Result<String> {
        self.github
            .get::<String>(&format!("/repos/{}/{}/commits/{}/status", self.owner, self.repo, sha))
    }
}

// representations


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

#[derive(Debug, Serialize)]
pub struct StatusOptions {
    state: State,
    #[serde(skip_serializing_if="Option::is_none")]
    target_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    context: Option<String>,
}

#[derive(Default)]
pub struct StatusBuilder {
    state: State,
    target_url: Option<String>,
    description: Option<String>,
    context: Option<String>,
}

impl StatusBuilder {
    pub fn new(state: State) -> StatusBuilder {
        StatusBuilder { state: state, ..Default::default() }
    }

    pub fn target_url<T>(&mut self, url: T) -> &mut StatusBuilder
        where T: Into<String>
    {
        self.target_url = Some(url.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut StatusBuilder
        where D: Into<String>
    {
        self.description = Some(desc.into());
        self
    }

    pub fn context<C>(&mut self, ctx: C) -> &mut StatusBuilder
        where C: Into<String>
    {
        self.context = Some(ctx.into());
        self
    }

    pub fn build(&self) -> StatusOptions {
        StatusOptions::new(self.state.clone(),
                           self.target_url.clone(),
                           self.description.clone(),
                           self.context.clone())
    }
}

impl StatusOptions {
    pub fn new<T, D, C>(state: State,
                        target_url: Option<T>,
                        descr: Option<D>,
                        context: Option<C>)
                        -> StatusOptions
        where T: Into<String>,
              D: Into<String>,
              C: Into<String>
    {
        StatusOptions {
            state: state,
            target_url: target_url.map(|t| t.into()),
            description: descr.map(|d| d.into()),
            context: context.map(|c| c.into()),
        }
    }

    pub fn builder(state: State) -> StatusBuilder {
        StatusBuilder::new(state)
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
    use serde::ser::Serialize;
    use super::*;
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
        for (json, value) in vec![("\"pending\"", State::Pending),
                                  ("\"success\"", State::Success),
                                  ("\"error\"", State::Error),
                                  ("\"failure\"", State::Failure)] {
            assert_eq!(serde_json::from_str::<State>(json).unwrap(), value)
        }
    }

    #[test]
    fn serialize_status_state() {
        for (json, value) in vec![("\"pending\"", State::Pending),
                                  ("\"success\"", State::Success),
                                  ("\"error\"", State::Error),
                                  ("\"failure\"", State::Failure)] {
            assert_eq!(serde_json::to_string(&value).unwrap(), json)
        }
    }


    #[test]
    fn status_reqs() {
        let tests = vec![(StatusOptions::builder(State::Pending).build(),
                  r#"{"state":"pending"}"#),
                 (StatusOptions::builder(State::Success)
                      .target_url("http://acme.com")
                      .build(),
                  r#"{"state":"success","target_url":"http://acme.com"}"#),
                 (StatusOptions::builder(State::Error).description("desc").build(),
                  r#"{"state":"error","description":"desc"}"#),
                 (StatusOptions::builder(State::Failure)
                      .target_url("http://acme.com")
                      .description("desc")
                      .build(),
                  r#"{"state":"failure","target_url":"http://acme.com","description":"desc"}"#)];
        test_encoding(tests)
    }


}
