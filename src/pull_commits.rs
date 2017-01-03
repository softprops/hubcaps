//! Pull Commits interface

use super::{Github, Result, Iter};
use rep::PullCommit;

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
