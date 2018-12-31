//! Deployments interface

extern crate futures;
extern crate serde_json;

use std::collections::HashMap;

use hyper::client::connect::Connect;
use serde;
use statuses::State;
use url::form_urlencoded;
use users::User;

use {Future, Github};

/// Interface for repository deployments
pub struct Deployments<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

/// Interface for deployment statuses
pub struct DeploymentStatuses<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
    id: u64,
}

impl<C: Clone + Connect + 'static> DeploymentStatuses<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, id: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        DeploymentStatuses {
            github,
            owner: owner.into(),
            repo: repo.into(),
            id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/deployments/{}/statuses{}",
            self.owner, self.repo, self.id, more
        )
    }

    /// lists all statuses associated with a deployment
    pub fn list(&self) -> Future<Vec<DeploymentStatus>> {
        self.github.get(&self.path(""))
    }

    /// creates a new deployment status. For convenience, a DeploymentStatusOptions.builder
    /// interface is required for building up a request
    pub fn create(&self, status: &DeploymentStatusOptions) -> Future<DeploymentStatus> {
        self.github.post(&self.path(""), json!(status))
    }
}

impl<C: Clone + Connect + 'static> Deployments<C> {
    /// Create a new deployments instance
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Deployments<C>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Deployments {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/deployments{}", self.owner, self.repo, more)
    }

    /// lists all deployments for a repository
    pub fn list(&self, opts: &DeploymentListOptions) -> Future<Vec<Deployment>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = opts.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// creates a new deployment for this repository
    pub fn create(&self, dep: &DeploymentOptions) -> Future<Deployment> {
        self.github.post(&self.path(""), json!(dep))
    }

    /// get a reference to the statuses api for a give deployment
    pub fn statuses(&self, id: u64) -> DeploymentStatuses<C> {
        DeploymentStatuses::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            id,
        )
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

#[derive(Debug, Default, Serialize)]
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

pub struct DeploymentOptionsBuilder(DeploymentOptions);

impl DeploymentOptionsBuilder {
    pub(crate) fn new<C>(commit: C) -> DeploymentOptionsBuilder
    where
        C: Into<String>,
    {
        DeploymentOptionsBuilder(DeploymentOptions {
            commit_ref: commit.into(),
            ..DeploymentOptions::default()
        })
    }

    pub fn task<T>(&mut self, task: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.0.task = Some(task.into());
        self
    }

    pub fn auto_merge(&mut self, auto_merge: bool) -> &mut Self {
        self.0.auto_merge = Some(auto_merge);
        self
    }

    pub fn required_contexts<C>(&mut self, ctxs: Vec<C>) -> &mut Self
    where
        C: Into<String>,
    {
        self.0.required_contexts =
            Some(ctxs.into_iter().map(|c| c.into()).collect::<Vec<String>>());
        self
    }

    pub fn payload<T: serde::ser::Serialize>(&mut self, pl: T) -> &mut Self {
        self.0.payload = serde_json::ser::to_string(&pl).ok();
        self
    }

    pub fn environment<E>(&mut self, env: E) -> &mut Self
    where
        E: Into<String>,
    {
        self.0.environment = Some(env.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut Self
    where
        D: Into<String>,
    {
        self.0.description = Some(desc.into());
        self
    }

    pub fn build(&self) -> DeploymentOptions {
        DeploymentOptions {
            commit_ref: self.0.commit_ref.clone(),
            task: self.0.task.clone(),
            auto_merge: self.0.auto_merge,
            required_contexts: self.0.required_contexts.clone(),
            payload: self.0.payload.clone(),
            environment: self.0.environment.clone(),
            description: self.0.description.clone(),
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

pub struct DeploymentStatusOptionsBuilder(DeploymentStatusOptions);

impl DeploymentStatusOptionsBuilder {
    pub(crate) fn new(state: State) -> DeploymentStatusOptionsBuilder {
        DeploymentStatusOptionsBuilder(DeploymentStatusOptions {
            state,
            ..Default::default()
        })
    }

    pub fn target_url<T>(&mut self, url: T) -> &mut DeploymentStatusOptionsBuilder
    where
        T: Into<String>,
    {
        self.0.target_url = Some(url.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut DeploymentStatusOptionsBuilder
    where
        D: Into<String>,
    {
        self.0.description = Some(desc.into());
        self
    }

    pub fn build(&self) -> DeploymentStatusOptions {
        DeploymentStatusOptions {
            state: self.0.state.clone(),
            target_url: self.0.target_url.clone(),
            description: self.0.description.clone(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
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
        DeploymentListOptionsBuilder::default()
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
pub struct DeploymentListOptionsBuilder(DeploymentListOptions);

impl DeploymentListOptionsBuilder {
    pub fn sha<S>(&mut self, s: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.0.params.insert("sha", s.into());
        self
    }

    pub fn commit_ref<G>(&mut self, r: G) -> &mut Self
    where
        G: Into<String>,
    {
        self.0.params.insert("ref", r.into());
        self
    }

    pub fn task<T>(&mut self, t: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.0.params.insert("task", t.into());
        self
    }

    pub fn environment<E>(&mut self, e: E) -> &mut Self
    where
        E: Into<String>,
    {
        self.0.params.insert("environment", e.into());
        self
    }

    pub fn build(&self) -> DeploymentListOptions {
        DeploymentListOptions {
            params: self.0.params.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeploymentOptions, DeploymentStatusOptions};
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
                r#"{"ref":"test"}"#,
            ),
            (
                DeploymentOptions::builder("test").task("launchit").build(),
                r#"{"ref":"test","task":"launchit"}"#,
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
                ),
            ),
        ];
        test_encoding(tests)
    }

    #[test]
    fn deployment_status_reqs() {
        let tests = vec![
            (
                DeploymentStatusOptions::builder(State::Pending).build(),
                r#"{"state":"pending"}"#,
            ),
            (
                DeploymentStatusOptions::builder(State::Pending)
                    .target_url("http://host.com")
                    .build(),
                r#"{"state":"pending","target_url":"http://host.com"}"#,
            ),
            (
                DeploymentStatusOptions::builder(State::Pending)
                    .target_url("http://host.com")
                    .description("desc")
                    .build(),
                r#"{"state":"pending","target_url":"http://host.com","description":"desc"}"#,
            ),
        ];
        test_encoding(tests)
    }
}
