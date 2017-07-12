//! Deployments interface
extern crate serde_json;

use std::collections::HashMap;
use url::form_urlencoded;
use serde;

use self::super::{Github, Result};
use statuses::State;
use users::User;

/// Interface for repository deployements
pub struct Deployments<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

/// INterface for deployment statuses
pub struct DeploymentStatuses<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    id: u64,
}

impl<'a> DeploymentStatuses<'a> {
    #[doc(hidden)]
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, id: u64) -> DeploymentStatuses<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        DeploymentStatuses {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            id: id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/deployments/{}/statuses{}",
            self.owner,
            self.repo,
            self.id,
            more
        )
    }

    /// lists all statuses associated with a deployment
    pub fn list(&self) -> Result<Vec<DeploymentStatus>> {
        self.github.get::<Vec<DeploymentStatus>>(&self.path(""))
    }

    /// creates a new deployment status. For convenience, a DeploymentStatusOptions.builder
    /// interface is required for building up a request
    pub fn create(&self, status: &DeploymentStatusOptions) -> Result<DeploymentStatus> {
        let data = serde_json::to_string(&status)?;
        self.github.post::<DeploymentStatus>(
            &self.path(""),
            &data.as_bytes(),
        )
    }
}

impl<'a> Deployments<'a> {
    /// Create a new deployments instance
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Deployments<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Deployments {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/deployments{}", self.owner, self.repo, more)
    }

    /// lists all deployments for a repository
    pub fn list(&self, opts: &DeploymentListOptions) -> Result<Vec<Deployment>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = opts.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Deployment>>(&uri.join("?"))
    }

    /// creates a new deployment for this repository
    pub fn create(&self, dep: &DeploymentOptions) -> Result<Deployment> {
        let data = serde_json::to_string(&dep)?;
        self.github.post::<Deployment>(
            &self.path(""),
            data.as_bytes(),
        )
    }

    /// get a reference to the statuses api for a give deployment
    pub fn statuses(&self, id: u64) -> DeploymentStatuses {
        DeploymentStatuses::new(self.github, self.owner.as_str(), self.repo.as_str(), id)
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Deployment {
    pub url: String,
    pub id: u64,
    pub sha: String,
    #[serde(rename = "ref")]
    pub commit_ref: String,
    pub task: String,
    pub payload: serde_json::Value,
    pub environment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub creator: User,
    pub created_at: String,
    pub updated_at: String,
    pub statuses_url: String,
    pub repository_url: String,
}

#[derive(Debug, Serialize)]
pub struct DeploymentOptions {
    #[serde(rename = "ref")]
    pub commit_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_contexts: Option<Vec<String>>,
    /// contents of payload should be valid JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DeploymentOptions {
    pub fn builder<C>(commit: C) -> DeploymentOptionsBuilder
    where
        C: Into<String>,
    {
        DeploymentOptionsBuilder::new(commit)
    }
}

#[derive(Default)]
pub struct DeploymentOptionsBuilder {
    pub commit_ref: String,
    pub task: Option<String>,
    pub auto_merge: Option<bool>,
    pub required_contexts: Option<Vec<String>>,
    pub payload: Option<String>,
    pub environment: Option<String>,
    pub description: Option<String>,
}

impl DeploymentOptionsBuilder {
    pub fn new<C>(commit: C) -> DeploymentOptionsBuilder
    where
        C: Into<String>,
    {
        DeploymentOptionsBuilder {
            commit_ref: commit.into(),
            ..Default::default()
        }
    }

    pub fn task<T>(&mut self, task: T) -> &mut DeploymentOptionsBuilder
    where
        T: Into<String>,
    {
        self.task = Some(task.into());
        self
    }

    pub fn auto_merge(&mut self, auto_merge: bool) -> &mut DeploymentOptionsBuilder {
        self.auto_merge = Some(auto_merge);
        self
    }

    pub fn required_contexts<C>(&mut self, ctxs: Vec<C>) -> &mut DeploymentOptionsBuilder
    where
        C: Into<String>,
    {
        self.required_contexts = Some(ctxs.into_iter().map(|c| c.into()).collect::<Vec<String>>());
        self
    }

    pub fn payload<T: serde::ser::Serialize>(&mut self, pl: T) -> &mut DeploymentOptionsBuilder {
        self.payload = serde_json::ser::to_string(&pl).ok();
        self
    }

    pub fn environment<E>(&mut self, env: E) -> &mut DeploymentOptionsBuilder
    where
        E: Into<String>,
    {
        self.environment = Some(env.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut DeploymentOptionsBuilder
    where
        D: Into<String>,
    {
        self.description = Some(desc.into());
        self
    }

    pub fn build(&self) -> DeploymentOptions {
        DeploymentOptions {
            commit_ref: self.commit_ref.clone(),
            task: self.task.clone(),
            auto_merge: self.auto_merge,
            required_contexts: self.required_contexts.clone(),
            payload: self.payload.clone(),
            environment: self.environment.clone(),
            description: self.description.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeploymentStatus {
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: State,
    pub target_url: Option<String>,
    pub description: Option<String>,
    pub id: u64,
    pub deployment_url: String,
    pub repository_url: String,
    pub creator: User,
}

#[derive(Default)]
pub struct DeploymentStatusOptionsBuilder {
    state: State,
    target_url: Option<String>,
    description: Option<String>,
}

impl DeploymentStatusOptionsBuilder {
    pub fn new(state: State) -> DeploymentStatusOptionsBuilder {
        DeploymentStatusOptionsBuilder {
            state: state,
            ..Default::default()
        }
    }

    pub fn target_url<T>(&mut self, url: T) -> &mut DeploymentStatusOptionsBuilder
    where
        T: Into<String>,
    {
        self.target_url = Some(url.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut DeploymentStatusOptionsBuilder
    where
        D: Into<String>,
    {
        self.description = Some(desc.into());
        self
    }

    pub fn build(&self) -> DeploymentStatusOptions {
        DeploymentStatusOptions {
            state: self.state.clone(),
            target_url: self.target_url.clone(),
            description: self.description.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DeploymentStatusOptions {
    state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl DeploymentStatusOptions {
    pub fn builder(state: State) -> DeploymentStatusOptionsBuilder {
        DeploymentStatusOptionsBuilder::new(state)
    }
}

#[derive(Default)]
pub struct DeploymentListOptions {
    params: HashMap<&'static str, String>,
}

impl DeploymentListOptions {
    /// return a new instance of a builder for options
    pub fn builder() -> DeploymentListOptionsBuilder {
        DeploymentListOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded)
        }
    }
}

#[derive(Default)]
pub struct DeploymentListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl DeploymentListOptionsBuilder {
    pub fn new() -> DeploymentListOptionsBuilder {
        DeploymentListOptionsBuilder { ..Default::default() }
    }

    pub fn sha<S>(&mut self, s: S) -> &mut DeploymentListOptionsBuilder
    where
        S: Into<String>,
    {
        self.params.insert("sha", s.into());
        self
    }

    pub fn commit_ref<G>(&mut self, r: G) -> &mut DeploymentListOptionsBuilder
    where
        G: Into<String>,
    {
        self.params.insert("ref", r.into());
        self
    }

    pub fn task<T>(&mut self, t: T) -> &mut DeploymentListOptionsBuilder
    where
        T: Into<String>,
    {
        self.params.insert("task", t.into());
        self
    }

    pub fn environment<E>(&mut self, e: E) -> &mut DeploymentListOptionsBuilder
    where
        E: Into<String>,
    {
        self.params.insert("environment", e.into());
        self
    }

    pub fn build(&self) -> DeploymentListOptions {
        DeploymentListOptions { params: self.params.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeploymentStatusOptions, DeploymentOptions};
    use serde::ser::Serialize;
    use serde_json;
    use statuses::State;
    use std::collections::BTreeMap;

    fn test_encoding<E: Serialize>(tests: Vec<(E, &str)>) {
        for test in tests {
            match test {
                (k, v) => assert_eq!(serde_json::to_string(&k).unwrap(), v),
            }
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
                DeploymentOptions::builder("topic-branch")
                    .description("description")
                    .payload(payload)
                    .build(),
                concat!(
                    "{",
                    r#""ref":"topic-branch","#,
                    r#""payload":"{\"room_id\":\"123456\",\"user\":\"atmos\"}","#,
                    r#""description":"description""#,
                    "}"
                )
            )
        ];
        test_encoding(tests)
    }

    #[test]
    fn deployment_status_reqs() {
        let tests = vec![
            (
                DeploymentStatusOptions::builder(State::Pending).build(),
                r#"{"state":"pending"}"#
            ),
            (
                DeploymentStatusOptions::builder(State::Pending)
                    .target_url("http://host.com")
                    .build(),
                r#"{"state":"pending","target_url":"http://host.com"}"#
            ),
            (
                DeploymentStatusOptions::builder(State::Pending)
                    .target_url("http://host.com")
                    .description("desc")
                    .build(),
                r#"{"state":"pending","target_url":"http://host.com","description":"desc"}"#
            ),
        ];
        test_encoding(tests)
    }
}
