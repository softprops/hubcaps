//! Issues interface

extern crate serde_json;

use super::{Github, Result};
use comments::Comments;
use users::User;
use labels::Label;
use super::SortDirection;
use std::fmt;
use std::collections::HashMap;
use url::form_urlencoded;

/// enum representation of github pull and issue state
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    /// Only open issues
    Open,
    /// Only closed issues
    Closed,
    /// All issues, open or closed
    All,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                State::Open => "open",
                State::Closed => "closed",
                State::All => "all",
            }
        )
    }
}

impl Default for State {
    fn default() -> State {
        State::Open
    }
}

/// Sort options available for github issues
#[derive(Clone, Debug, PartialEq)]
pub enum Sort {
    /// sort by creation time of issue
    Created,
    /// sort by the last time issue was updated
    Updated,
    /// sort by number of comments
    Comments,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Sort::Created => "created",
                Sort::Updated => "updated",
                Sort::Comments => "comments",
            }
        )
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

/// Provides access to label operations available for an individual issues
pub struct IssueLabels<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> IssueLabels<'a> {
    #[doc(hidden)]
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, number: u64) -> IssueLabels<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        IssueLabels {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}/labels{}",
            self.owner,
            self.repo,
            self.number,
            more
        )
    }

    /// add a set of labels to this issue ref
    pub fn add(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
        self.github.post::<Vec<Label>>(
            &self.path(""),
            serde_json::to_string(&labels)?.as_bytes(),
        )
    }

    /// remove a label from this issue
    pub fn remove(&self, label: &str) -> Result<()> {
        self.github.delete(&self.path(&format!("/{}", label)))
    }

    /// replace all labels associated with this issue with a new set.
    /// providing an empty set of labels is the same as clearing the
    /// current labels
    pub fn set(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
        self.github.put::<Vec<Label>>(
            &self.path(""),
            serde_json::to_string(&labels)?.as_bytes(),
        )
    }

    /// remove all labels from an issue
    pub fn clear(&self) -> Result<()> {
        self.github.delete(&self.path(""))
    }
}

/// Provides access to operations available for a single issue
/// Typically accessed from `github.repo(.., ..).issues().get(number)`
pub struct IssueRef<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> IssueRef<'a> {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, number: u64) -> IssueRef<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        IssueRef {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}{}",
            self.owner,
            self.repo,
            self.number,
            more
        )
    }

    pub fn labels(&self) -> IssueLabels {
        IssueLabels::new(
            self.github,
            self.owner.as_str(),
            self.repo.as_str(),
            self.number,
        )
    }

    pub fn edit(&self, is: &IssueOptions) -> Result<Issue> {
        let data = serde_json::to_string(&is)?;
        self.github.patch::<Issue>(&self.path(""), data.as_bytes())
    }

    pub fn comments(&self) -> Comments {
        Comments::new(
            self.github,
            self.owner.clone(),
            self.repo.clone(),
            self.number,
        )
    }
}

/// Provides access to operations available for a repository issues
/// Typically accessed via `github.repo(..., ...).issues()`
pub struct Issues<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Issues<'a> {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Issues<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Issues {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues{}", self.owner, self.repo, more)
    }

    pub fn get(&self, number: u64) -> IssueRef {
        IssueRef::new(self.github, self.owner.as_str(), self.repo.as_str(), number)
    }

    pub fn create(&self, is: &IssueOptions) -> Result<Issue> {
        let data = serde_json::to_string(&is)?;
        self.github.post::<Issue>(&self.path(""), data.as_bytes())
    }

    pub fn list(&self, options: &IssueListOptions) -> Result<Vec<Issue>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Issue>>(&uri.join("?"))
    }
}

// representations

// todo: simplify with param
#[derive(Default)]
pub struct IssueListOptions {
    params: HashMap<&'static str, String>,
}

impl IssueListOptions {
    pub fn builder() -> IssueListOptionsBuilder {
        IssueListOptionsBuilder::new()
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
pub struct IssueListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl IssueListOptionsBuilder {
    pub fn new() -> IssueListOptionsBuilder {
        IssueListOptionsBuilder { ..Default::default() }
    }

    pub fn state(&mut self, state: State) -> &mut IssueListOptionsBuilder {
        self.params.insert("state", state.to_string());
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut IssueListOptionsBuilder {
        self.params.insert("sort", sort.to_string());
        self
    }

    pub fn asc(&mut self) -> &mut IssueListOptionsBuilder {
        self.direction(SortDirection::Asc)
    }

    pub fn desc(&mut self) -> &mut IssueListOptionsBuilder {
        self.direction(SortDirection::Desc)
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut IssueListOptionsBuilder {
        self.params.insert("direction", direction.to_string());
        self
    }

    pub fn assignee<A>(&mut self, assignee: A) -> &mut IssueListOptionsBuilder
    where
        A: Into<String>,
    {
        self.params.insert("assignee", assignee.into());
        self
    }

    pub fn creator<C>(&mut self, creator: C) -> &mut IssueListOptionsBuilder
    where
        C: Into<String>,
    {
        self.params.insert("creator", creator.into());
        self
    }

    pub fn mentioned<M>(&mut self, mentioned: M) -> &mut IssueListOptionsBuilder
    where
        M: Into<String>,
    {
        self.params.insert("mentioned", mentioned.into());
        self
    }

    pub fn labels<L>(&mut self, labels: Vec<L>) -> &mut IssueListOptionsBuilder
    where
        L: Into<String>,
    {
        self.params.insert(
            "labels",
            labels
                .into_iter()
                .map(|l| l.into())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }

    pub fn since<S>(&mut self, since: S) -> &mut IssueListOptionsBuilder
    where
        S: Into<String>,
    {
        self.params.insert("since", since.into());
        self
    }

    pub fn build(&self) -> IssueListOptions {
        IssueListOptions { params: self.params.clone() }
    }
}

#[derive(Debug, Serialize)]
pub struct IssueOptions {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<u64>,
    pub labels: Vec<String>,
}

impl IssueOptions {
    pub fn new<T, B, A, L>(
        title: T,
        body: Option<B>,
        assignee: Option<A>,
        milestone: Option<u64>,
        labels: Vec<L>,
    ) -> IssueOptions
    where
        T: Into<String>,
        B: Into<String>,
        A: Into<String>,
        L: Into<String>,
    {
        IssueOptions {
            title: title.into(),
            body: body.map(|b| b.into()),
            assignee: assignee.map(|a| a.into()),
            milestone: milestone,
            labels: labels
                .into_iter()
                .map(|l| l.into())
                .collect::<Vec<String>>(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub body: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub assignee: Option<User>,
    pub locked: bool,
    pub comments: u64,
    pub closed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state() {
        let default: State = Default::default();
        assert_eq!(default, State::Open)
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
            (IssueListOptions::builder().build(), None),
            (
                IssueListOptions::builder().state(State::Closed).build(),
                Some("state=closed".to_owned())
            ),
            (
                IssueListOptions::builder()
                    .labels(vec!["foo", "bar"])
                    .build(),
                Some("labels=foo%2Cbar".to_owned())
            ),
        ];
        test_serialize(tests)
    }

    #[test]
    fn sort_default() {
        let default: Sort = Default::default();
        assert_eq!(default, Sort::Created)
    }

    #[test]
    fn sort_display() {
        for (k, v) in vec![
            (Sort::Created, "created"),
            (Sort::Updated, "updated"),
            (Sort::Comments, "comments"),
        ]
        {
            assert_eq!(k.to_string(), v)
        }
    }
}
