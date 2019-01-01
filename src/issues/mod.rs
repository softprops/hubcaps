//! Issues interface

use std::collections::HashMap;
use std::fmt;

use hyper::client::connect::Connect;
use url::form_urlencoded;

use comments::Comments;
use labels::Label;
use users::User;
use {serde_json, unfold, Future, Github, SortDirection, Stream};

/// enum representation of github pull and issue state
#[derive(Clone, Copy, Debug, PartialEq)]
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
        match *self {
            State::Open => "open",
            State::Closed => "closed",
            State::All => "all",
        }
        .fmt(f)
    }
}

impl Default for State {
    fn default() -> State {
        State::Open
    }
}

/// Sort options available for github issues
#[derive(Clone, Copy, Debug, PartialEq)]
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
        match *self {
            Sort::Created => "created",
            Sort::Updated => "updated",
            Sort::Comments => "comments",
        }
        .fmt(f)
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

/// Provides access to assignee operations available for an individual issue
pub struct IssueAssignees<C: Clone + Connect + 'static> {
    github: Github<C>,
    owner: String,
    repo: String,
    number: u64,
}

impl<C: Clone + Connect + 'static> IssueAssignees<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        IssueAssignees {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}/assignees{}",
            self.owner, self.repo, self.number, more
        )
    }

    /// add a set of assignees
    pub fn add(&self, assignees: Vec<&str>) -> Future<Issue> {
        self.github.post(&self.path(""), json_lit!({ "assignees": assignees }))
    }
}

/// Provides access to label operations available for an individual issue
pub struct IssueLabels<C: Clone + Connect + 'static> {
    github: Github<C>,
    owner: String,
    repo: String,
    number: u64,
}

impl<C: Clone + Connect + 'static> IssueLabels<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        IssueLabels {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}/labels{}",
            self.owner, self.repo, self.number, more
        )
    }

    /// add a set of labels to this issue ref
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))] // shippied public API
    pub fn add(&self, labels: Vec<&str>) -> Future<Vec<Label>> {
        self.github.post(&self.path(""), json!(labels))
    }

    /// remove a label from this issue
    pub fn remove(&self, label: &str) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}", label)))
    }

    /// replace all labels associated with this issue with a new set.
    /// providing an empty set of labels is the same as clearing the
    /// current labels
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))] // shippied public API
    pub fn set(&self, labels: Vec<&str>) -> Future<Vec<Label>> {
        self.github.put(&self.path(""), json!(labels))
    }

    /// remove all labels from an issue
    pub fn clear(&self) -> Future<()> {
        self.github.delete(&self.path(""))
    }
}

/// Provides access to operations available for a single issue
/// Typically accessed from `github.repo(.., ..).issues().get(number)`
pub struct IssueRef<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
    number: u64,
}

impl<C: Clone + Connect + 'static> IssueRef<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        IssueRef {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    /// Request an issue's information
    pub fn get(&self) -> Future<Issue> {
        self.github.get(&self.path(""))
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}{}",
            self.owner, self.repo, self.number, more
        )
    }

    /// Return a reference to labels operations available for this issue
    pub fn labels(&self) -> IssueLabels<C> {
        IssueLabels::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            self.number,
        )
    }

    /// Return a reference to assignee operations available for this issue
    pub fn assignees(&self) -> IssueAssignees<C> {
        IssueAssignees::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            self.number,
        )
    }

    /// Edit the issues options
    pub fn edit(&self, is: &IssueOptions) -> Future<Issue> {
        self.github.patch(&self.path(""), json!(is))
    }

    /// Return a reference to comment operations available for this issue
    pub fn comments(&self) -> Comments<C> {
        Comments::new(
            self.github.clone(),
            self.owner.clone(),
            self.repo.clone(),
            self.number,
        )
    }
}

/// Provides access to operations available for a repository issues
/// Typically accessed via `github.repo(..., ...).issues()`
pub struct Issues<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

fn identity<T>(x: T) -> T {
    x
}

impl<C: Clone + Connect + 'static> Issues<C> {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Issues {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues{}", self.owner, self.repo, more)
    }

    pub fn get(&self, number: u64) -> IssueRef<C> {
        IssueRef::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            number,
        )
    }

    pub fn create(&self, is: &IssueOptions) -> Future<Issue> {
        self.github.post(&self.path(""), json!(is))
    }

    /// Return the first page of issues for this repisotiry
    /// See the [github docs](https://developer.github.com/v3/issues/#list-issues-for-a-repository)
    /// for more information
    pub fn list(&self, options: &IssueListOptions) -> Future<Vec<Issue>> {
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
    pub fn iter(&self, options: &IssueListOptions) -> Stream<Issue> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        unfold(
            self.github.clone(),
            self.github.get_pages(&uri.join("?")),
            identity,
        )
    }
}

// representations

/// Options used to filter repository issue listings
///
/// See the [github docs](https://developer.github.com/v3/issues/#list-issues-for-a-repository)
/// for more information
///
/// By default this returns up to `30` items. You can
/// request up to `100` using the [per_page](https://developer.github.com/v3/#pagination)
/// parameter
#[derive(Default)]
pub struct IssueListOptions {
    params: HashMap<&'static str, String>,
}

impl IssueListOptions {
    pub fn builder() -> IssueListOptionsBuilder {
        IssueListOptionsBuilder::default()
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
pub struct IssueListOptionsBuilder(IssueListOptions);

impl IssueListOptionsBuilder {
    pub fn state(&mut self, state: State) -> &mut Self {
        self.0.params.insert("state", state.to_string());
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.0.params.insert("sort", sort.to_string());
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.direction(SortDirection::Asc)
    }

    pub fn desc(&mut self) -> &mut Self {
        self.direction(SortDirection::Desc)
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut Self {
        self.0.params.insert("direction", direction.to_string());
        self
    }

    pub fn assignee<A>(&mut self, assignee: A) -> &mut Self
    where
        A: Into<String>,
    {
        self.0.params.insert("assignee", assignee.into());
        self
    }

    pub fn creator<C>(&mut self, creator: C) -> &mut Self
    where
        C: Into<String>,
    {
        self.0.params.insert("creator", creator.into());
        self
    }

    pub fn mentioned<M>(&mut self, mentioned: M) -> &mut Self
    where
        M: Into<String>,
    {
        self.0.params.insert("mentioned", mentioned.into());
        self
    }

    pub fn labels<L>(&mut self, labels: Vec<L>) -> &mut Self
    where
        L: Into<String>,
    {
        self.0.params.insert(
            "labels",
            labels
                .into_iter()
                .map(|l| l.into())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }

    pub fn since<S>(&mut self, since: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.0.params.insert("since", since.into());
        self
    }

    pub fn per_page(&mut self, n: u32) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn build(&self) -> IssueListOptions {
        IssueListOptions {
            params: self.0.params.clone(),
        }
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
            milestone,
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
    pub body: Option<String>,
    pub user: User,
    pub labels: Vec<Label>,
    pub assignee: Option<User>,
    pub locked: bool,
    pub comments: u64,
    pub pull_request: Option<PullRef>,
    pub closed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub assignees: Vec<User>,
}

/// A reference to a pull request.
#[derive(Debug, Deserialize)]
pub struct PullRef {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
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
                Some("state=closed".to_owned()),
            ),
            (
                IssueListOptions::builder()
                    .labels(vec!["foo", "bar"])
                    .build(),
                Some("labels=foo%2Cbar".to_owned()),
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
        for (k, v) in &[
            (Sort::Created, "created"),
            (Sort::Updated, "updated"),
            (Sort::Comments, "comments"),
        ] {
            assert_eq!(k.to_string(), *v)
        }
    }
}
