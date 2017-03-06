//! Rust representations of Github API data structures
use users::User;


#[derive(Debug, Deserialize)]
pub struct Commit {
    pub label: String,
    #[serde(rename="ref")]
    pub commit_ref: String,
    pub sha: String,
    pub user: User, //    pub repo: Option<Repo>,
}


#[derive(Debug, Deserialize)]
pub struct Status {
    pub created_at: String,
    pub updated_at: String,
    pub state: StatusState,
    pub target_url: String,
    pub description: String,
    pub id: u64,
    pub url: String,
    pub context: String,
    pub creator: User,
}

#[derive(Debug, Serialize)]
pub struct StatusOptions {
    state: StatusState,
    #[serde(skip_serializing_if="Option::is_none")]
    target_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    context: Option<String>,
}

#[derive(Default)]
pub struct StatusBuilder {
    state: StatusState,
    target_url: Option<String>,
    description: Option<String>,
    context: Option<String>,
}

impl StatusBuilder {
    pub fn new(state: StatusState) -> StatusBuilder {
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
    pub fn new<T, D, C>(state: StatusState,
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

    pub fn builder(state: StatusState) -> StatusBuilder {
        StatusBuilder::new(state)
    }
}

#[derive(Debug, Deserialize)]
pub struct UserStamp {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitRef {
    pub url: String,
    pub sha: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum StatusState {
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

impl Default for StatusState {
    fn default() -> StatusState {
        StatusState::Pending
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
        for (json, value) in vec![("\"pending\"", StatusState::Pending),
                                  ("\"success\"", StatusState::Success),
                                  ("\"error\"", StatusState::Error),
                                  ("\"failure\"", StatusState::Failure)] {
            assert_eq!(serde_json::from_str::<StatusState>(json).unwrap(), value)
        }
    }

    #[test]
    fn serialize_status_state() {
        for (json, value) in vec![("\"pending\"", StatusState::Pending),
                                  ("\"success\"", StatusState::Success),
                                  ("\"error\"", StatusState::Error),
                                  ("\"failure\"", StatusState::Failure)] {
            assert_eq!(serde_json::to_string(&value).unwrap(), json)
        }
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


}
