use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::form_urlencoded;

use crate::{Future, Github, SortDirection, Stream};
use crate::issues::State;
use crate::users::User;

pub struct Milestones {
    github: Github,
    owner: String,
    repo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: u64,
    pub node_id: String,
    pub number: u64,
    pub title: String,
    pub description: Option<String>,
    pub creator: User,
    pub open_issues: u64,
    pub closed_issues: u64,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub due_on: Option<String>,
    pub closed_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MilestoneOptions {
    pub title: String,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<String>,
}

#[derive(Default)]
pub struct MilestoneListOptions {
    params: HashMap<&'static str, String>,
}

impl MilestoneListOptions {
    pub fn builder() -> MilestoneListOptionsBuilder {
        MilestoneListOptionsBuilder::default()
    }

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

/// a mutable issue list builder
#[derive(Default)]
pub struct MilestoneListOptionsBuilder(MilestoneListOptions);

impl MilestoneListOptionsBuilder {
    pub fn state(&mut self, state: State) -> &mut Self {
        self.0.params.insert("state", state.to_string());
        self
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut Self {
        self.0.params.insert("direction", direction.to_string());
        self
    }

    pub fn per_page(&mut self, n: u32) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn build(&self) -> MilestoneListOptions {
        MilestoneListOptions {
            params: self.0.params.clone(),
        }
    }
}

impl Milestones {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
        where
            O: Into<String>,
            R: Into<String>,
    {
        Milestones {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/milestones{}", self.owner, self.repo, more)
    }

    pub fn create(&self, is: &MilestoneOptions) -> Future<Milestone> {
        self.github.post(&self.path(""), json!(is))
    }

    pub fn update(&self, is: &MilestoneOptions) -> Future<Milestone> {
        self.github.patch(&self.path(""), json!(is))
    }

    /// Return the first page of issues for this repisotiry
    /// See the [github docs](https://developer.github.com/v3/issues/#list-issues-for-a-repository)
    /// for more information
    pub fn list(&self, options: &MilestoneListOptions) -> Future<Vec<Milestone>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// Return a stream of all issues for this repository
    ///
    /// See the [github docs](https://developer.github.com/v3/issues/#list-issues-for-a-repository)
    /// for more information
    ///
    /// Note: You'll typically want to use a `IssueListOptions` with a `per_page`
    /// of 100 for maximum api credential rate limit efficency
    pub fn iter(&self, options: &MilestoneListOptions) -> Stream<Milestone> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get_stream(&uri.join("?"))
    }
}