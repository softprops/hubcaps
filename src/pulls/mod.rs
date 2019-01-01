//! Pull requests interface

use std::collections::HashMap;
use std::fmt;

use hyper::client::connect::Connect;
use serde_json;
use url::form_urlencoded;

use comments::Comments;
use issues::{IssueAssignees, IssueLabels, Sort as IssueSort, State};
use labels::Label;
use pull_commits::PullCommits;
use review_comments::ReviewComments;
use users::User;
use {unfold, Future, Github, SortDirection, Stream};

fn identity<T>(x: T) -> T {
    x
}

/// Sort directions for pull requests
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sort {
    /// Sort by time created
    Created,
    /// Sort by last updated
    Updated,
    /// Sort by popularity
    Popularity,
    /// Sort by long running issues
    LongRunning,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sort::Created => "created",
            Sort::Updated => "updated",
            Sort::Popularity => "popularity",
            Sort::LongRunning => "long-running",
        }
        .fmt(f)
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

/// A structure for accessing interfacing with a specific pull request
pub struct PullRequest<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
    number: u64,
}

impl<C: Clone + Connect + 'static> PullRequest<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        PullRequest {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/pulls/{}{}",
            self.owner, self.repo, self.number, more
        )
    }

    /// Request a pull requests information
    pub fn get(&self) -> Future<Pull> {
        self.github.get(&self.path(""))
    }

    /// Return a reference to labels operations available for this pull request
    pub fn labels(&self) -> IssueLabels<C> {
        IssueLabels::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            self.number,
        )
    }

    /// Return a reference to assignee operations available for this pull request
    pub fn assignees(&self) -> IssueAssignees<C> {
        IssueAssignees::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            self.number,
        )
    }

    /// short hand for editing state = open
    pub fn open(&self) -> Future<Pull> {
        self.edit(&PullEditOptions::builder().state("open").build())
    }

    /// shorthand for editing state = closed
    pub fn close(&self) -> Future<Pull> {
        self.edit(&PullEditOptions::builder().state("closed").build())
    }

    /// Edit a pull request
    pub fn edit(&self, pr: &PullEditOptions) -> Future<Pull> {
        self.github.patch::<Pull>(&self.path(""), json!(pr))
    }

    /// Returns a vector of file diffs associated with this pull
    pub fn files(&self) -> Future<Vec<FileDiff>> {
        self.github.get(&self.path("/files"))
    }

    /// returns issue comments interface
    pub fn comments(&self) -> Comments<C> {
        Comments::new(
            self.github.clone(),
            self.owner.clone(),
            self.repo.clone(),
            self.number,
        )
    }

    /// returns review comments interface
    pub fn review_comments(&self) -> ReviewComments<C> {
        ReviewComments::new(
            self.github.clone(),
            self.owner.clone(),
            self.repo.clone(),
            self.number,
        )
    }

    /// returns pull commits interface
    pub fn commits(&self) -> PullCommits<C> {
        PullCommits::new(
            self.github.clone(),
            self.owner.clone(),
            self.repo.clone(),
            self.number,
        )
    }
}

/// A structure for interfacing with a repositories list of pull requests
pub struct PullRequests<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> PullRequests<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        PullRequests {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/pulls{}", self.owner, self.repo, more)
    }

    /// Get a reference to a structure for interfacing with a specific pull request
    pub fn get(&self, number: u64) -> PullRequest<C> {
        PullRequest::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            number,
        )
    }

    /// Create a new pull request
    pub fn create(&self, pr: &PullOptions) -> Future<Pull> {
        self.github.post(&self.path(""), json!(pr))
    }

    /// list pull requests
    pub fn list(&self, options: &PullListOptions) -> Future<Vec<Pull>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Pull>>(&uri.join("?"))
    }

    /// provides a stream over all pages of pull requests
    pub fn iter(&self, options: &PullListOptions) -> Stream<Pull> {
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

// representations (todo: replace with derive_builder)

/// representation of a github pull request
#[derive(Debug, Deserialize)]
pub struct Pull {
    pub id: u64,
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub head: Commit,
    pub base: Commit,
    // links
    pub user: User,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub merge_commit_sha: Option<String>,
    pub mergeable: Option<bool>,
    pub merged_by: Option<User>,
    pub comments: Option<u64>,
    pub commits: Option<u64>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
    pub changed_files: Option<u64>,
    pub labels: Vec<Label>,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub label: String,
    #[serde(rename = "ref")]
    pub commit_ref: String,
    pub sha: String,
    pub user: User, //    pub repo: Option<Repo>,
}

#[derive(Default)]
pub struct PullEditOptionsBuilder(PullEditOptions);

impl PullEditOptionsBuilder {
    /// set the title of the pull
    pub fn title<T>(&mut self, title: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.0.title = Some(title.into());
        self
    }

    /// set the body of the pull
    pub fn body<B>(&mut self, body: B) -> &mut Self
    where
        B: Into<String>,
    {
        self.0.body = Some(body.into());
        self
    }

    /// set the state of the pull
    pub fn state<S>(&mut self, state: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.0.state = Some(state.into());
        self
    }

    /// create a new set of pull edit options
    pub fn build(&self) -> PullEditOptions {
        PullEditOptions {
            title: self.0.title.clone(),
            body: self.0.body.clone(),
            state: self.0.state.clone(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct PullEditOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

impl PullEditOptions {
    // todo represent state as enum
    pub fn new<T, B, S>(title: Option<T>, body: Option<B>, state: Option<S>) -> PullEditOptions
    where
        T: Into<String>,
        B: Into<String>,
        S: Into<String>,
    {
        PullEditOptions {
            title: title.map(|t| t.into()),
            body: body.map(|b| b.into()),
            state: state.map(|s| s.into()),
        }
    }
    pub fn builder() -> PullEditOptionsBuilder {
        PullEditOptionsBuilder::default()
    }
}

#[derive(Debug, Serialize)]
pub struct PullOptions {
    pub title: String,
    pub head: String,
    pub base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl PullOptions {
    pub fn new<T, H, BS, B>(title: T, head: H, base: BS, body: Option<B>) -> PullOptions
    where
        T: Into<String>,
        H: Into<String>,
        BS: Into<String>,
        B: Into<String>,
    {
        PullOptions {
            title: title.into(),
            head: head.into(),
            base: base.into(),
            body: body.map(|b| b.into()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileDiff {
    /// sha from GitHub may be null when file mode changed without contents changing
    pub sha: Option<String>,
    pub filename: String,
    pub status: String,
    pub additions: u64,
    pub deletions: u64,
    pub changes: u64,
    pub blob_url: String,
    pub raw_url: String,
    pub contents_url: String,
    /// patch is typically None for binary files
    pub patch: Option<String>,
}

#[derive(Default)]
pub struct PullListOptions {
    params: HashMap<&'static str, String>,
}

impl PullListOptions {
    pub fn builder() -> PullListOptionsBuilder {
        PullListOptionsBuilder::default()
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
pub struct PullListOptionsBuilder(PullListOptions);

impl PullListOptionsBuilder {
    pub fn state(&mut self, state: State) -> &mut Self {
        self.0.params.insert("state", state.to_string());
        self
    }

    pub fn sort(&mut self, sort: IssueSort) -> &mut Self {
        self.0.params.insert("sort", sort.to_string());
        self
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut Self {
        self.0.params.insert("direction", direction.to_string());
        self
    }

    pub fn build(&self) -> PullListOptions {
        PullListOptions {
            params: self.0.params.clone(),
        }
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
    fn pull_list_reqs() {
        fn test_serialize(tests: Vec<(PullListOptions, Option<String>)>) {
            for test in tests {
                match test {
                    (k, v) => assert_eq!(k.serialize(), v),
                }
            }
        }
        let tests = vec![
            (PullListOptions::builder().build(), None),
            (
                PullListOptions::builder().state(State::Closed).build(),
                Some("state=closed".to_owned()),
            ),
        ];
        test_serialize(tests)
    }

    #[test]
    fn pullreq_edits() {
        let tests = vec![
            (
                PullEditOptions::builder().title("test").build(),
                r#"{"title":"test"}"#,
            ),
            (
                PullEditOptions::builder()
                    .title("test")
                    .body("desc")
                    .build(),
                r#"{"title":"test","body":"desc"}"#,
            ),
            (
                PullEditOptions::builder().state("closed").build(),
                r#"{"state":"closed"}"#,
            ),
        ];
        test_encoding(tests)
    }

    #[test]
    fn default_sort() {
        let default: Sort = Default::default();
        assert_eq!(default, Sort::Created)
    }
}
