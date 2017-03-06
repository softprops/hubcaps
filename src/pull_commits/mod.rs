//! Pull Commits interface

use super::{Github, Result, Iter};
use users::User;

fn identity<T>(x: T) -> T {
    x
}

/// A structure for interfacing with a pull commits
pub struct PullCommits<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> PullCommits<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, number: u64) -> PullCommits<'a>
        where O: Into<String>,
              R: Into<String>
    {
        PullCommits {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    /// list pull commits
    pub fn list(&self) -> Result<Vec<PullCommit>> {
        let uri = format!("/repos/{}/{}/pulls/{}/commits",
                          self.owner,
                          self.repo,
                          self.number);
        self.github.get::<Vec<PullCommit>>(&uri)
    }

    // provides an iterator over all pages of pull commits
    pub fn iter(&'a self) -> Result<Iter<'a, Vec<PullCommit>, PullCommit>> {
        let uri = format!("/repos/{}/{}/pulls/{}/commits",
                          self.owner,
                          self.repo,
                          self.number);
        self.github.iter(uri, identity)
    }
}

// representations

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

#[derive(Debug, Deserialize)]
pub struct CommitDetails {
    pub url: String,
    pub author: UserStamp,
    pub committer: Option<UserStamp>,
    pub message: String,
    pub tree: CommitRef,
    pub comment_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct CommitRef {
    pub url: String,
    pub sha: String,
}

#[derive(Debug, Deserialize)]
pub struct UserStamp {
    pub name: String,
    pub email: String,
    pub date: String,
}
