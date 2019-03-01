//! Pull Commits interface
use serde::Deserialize;

use crate::users::User;
use crate::{Future, Github, Stream};

/// A structure for interfacing with a pull commits
pub struct PullCommits {
    github: Github,
    owner: String,
    repo: String,
    number: u64,
}

impl PullCommits {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        PullCommits {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    /// list pull commits
    pub fn list(&self) -> Future<Vec<PullCommit>> {
        let uri = format!(
            "/repos/{}/{}/pulls/{}/commits",
            self.owner, self.repo, self.number
        );
        self.github.get::<Vec<PullCommit>>(&uri)
    }

    /// provides a stream over all pages of pull commits
    pub fn iter(&self) -> Stream<PullCommit> {
        self.github.get_stream(&format!(
            "/repos/{}/{}/pulls/{}/commits",
            self.owner, self.repo, self.number
        ))
    }
}

// representations

/// Representation of a pull request commit
#[derive(Debug, Deserialize)]
pub struct PullCommit {
    pub url: String,
    pub sha: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: CommitDetails,
    pub author: User,
    pub committer: User,
    pub parents: Vec<CommitRef>,
}

/// Representation of a pull request commit details
#[derive(Debug, Deserialize)]
pub struct CommitDetails {
    pub url: String,
    pub author: UserStamp,
    pub committer: Option<UserStamp>,
    pub message: String,
    pub tree: CommitRef,
    pub comment_count: u64,
}

/// Representation of a reference to a commit
#[derive(Debug, Deserialize)]
pub struct CommitRef {
    pub url: String,
    pub sha: String,
}

/// Representation of a git user
#[derive(Debug, Deserialize)]
pub struct UserStamp {
    pub name: String,
    pub email: String,
    pub date: String,
}
