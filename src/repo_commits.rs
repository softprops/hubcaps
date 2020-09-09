//! Repo Commits interface
//! https://developer.github.com/v3/repos/commits/#get-a-single-commit
use serde::Deserialize;

use crate::users::User;
use crate::{Future, Github, Stream};

/// A structure for interfacing with a repository commits
pub struct RepoCommits {
    github: Github,
    owner: String,
    repo: String,
}

impl RepoCommits {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        RepoCommits {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list repo commits
    /// !!! make optional parameters
    pub fn list(&self, path: &str) -> Future<Vec<RepoCommit>> {
        let mut uri = format!("/repos/{}/{}/commits", self.owner, self.repo);
        if !path.is_empty() {
            uri += &format!("?path={}", path);
        }
        self.github.get::<Vec<RepoCommit>>(&uri)
    }

    /// provides a stream over all pages of pull commits
    /// !!! make optional parameters
    pub fn iter(&self) -> Stream<RepoCommit> {
        self.github
            .get_stream(&format!("/repos/{}/{}/commits", self.owner, self.repo))
    }

    /// get a repo commit
    pub fn get(&self, commit_ref: &str) -> Future<RepoCommit> {
        let uri = format!("/repos/{}/{}/commits/{}", self.owner, self.repo, commit_ref);
        self.github.get::<RepoCommit>(&uri)
    }
}

// representations

// !!! RepoCommit, CommitDetails, CommitRef, UserStamp are exact
//     dupes of pull_commits.rs' representations.

/// Representation of a repo commit
#[derive(Debug, Deserialize)]
pub struct RepoCommit {
    pub url: String,
    pub sha: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: CommitDetails,
    pub author: User,
    pub committer: User,
    pub parents: Vec<CommitRef>,
}

/// Representation of a repo commit details
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
