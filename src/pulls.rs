//! Pull requests interface
extern crate serde_json;

use super::{Github, Result};
use comments::Comments;
use review_comments::ReviewComments;
use rep::{FileDiff, Pull, PullEditOptions, PullOptions, PullListOptions};
use std::default::Default;
use std::fmt;

/// Sort directions for pull requests
#[derive(Debug, PartialEq)]
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
        write!(f,
               "{}",
               match *self {
                   Sort::Created => "created",
                   Sort::Updated => "updated",
                   Sort::Popularity => "popularity",
                   Sort::LongRunning => "long-running",
               })
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

/// A structure for accessing interfacing with a specific pull request
pub struct PullRequest<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> PullRequest<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, number: u64) -> PullRequest<'a>
        where O: Into<String>,
              R: Into<String>
    {
        PullRequest {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/pulls/{}{}",
                self.owner,
                self.repo,
                self.number,
                more)
    }

    /// Request a pull requests information
    pub fn get(&self) -> Result<Pull> {
        self.github.get::<Pull>(&self.path(""))
    }

    /// short hand for editing state = open
    pub fn open(&self) -> Result<Pull> {
        self.edit(&PullEditOptions::builder().state("open").build())
    }

    /// shorthand for editing state = closed
    pub fn close(&self) -> Result<Pull> {
        self.edit(&PullEditOptions::builder().state("closed").build())
    }

    /// Edit a pull request
    pub fn edit(&self, pr: &PullEditOptions) -> Result<Pull> {
        let data = try!(serde_json::to_string(&pr));
        self.github.patch::<Pull>(&self.path(""), data.as_bytes())
    }

    pub fn files(&self) -> Result<Vec<FileDiff>> {
        self.github.get::<Vec<FileDiff>>(&self.path("/files"))
    }

    /// returns issue comments interface
    pub fn comments(&self) -> Comments {
        Comments::new(self.github,
                      self.owner.clone(),
                      self.repo.clone(),
                      self.number)
    }

    /// returns review comments interface
    pub fn review_comments(&self) -> ReviewComments {
        ReviewComments::new(self.github,
                            self.owner.clone(),
                            self.repo.clone(),
                            self.number)
    }
}

/// A structure for interfacing with a repositories list of pull requests
pub struct PullRequests<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> PullRequests<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> PullRequests<'a>
        where O: Into<String>,
              R: Into<String>
    {
        PullRequests {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/pulls{}", self.owner, self.repo, more)
    }

    /// Get a reference to a strucuture for interfacing with a specific pull request
    pub fn get(&self, number: u64) -> PullRequest {
        PullRequest::new(self.github, self.owner.as_str(), self.repo.as_str(), number)
    }

    /// Create a new pull request
    pub fn create(&self, pr: &PullOptions) -> Result<Pull> {
        let data = try!(serde_json::to_string(&pr));
        self.github.post::<Pull>(&self.path(""), data.as_bytes())
    }

    /// list pull requests
    pub fn list(&self, options: &PullListOptions) -> Result<Vec<Pull>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Pull>>(&uri.join("?"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_sort() {
        let default: Sort = Default::default();
        assert_eq!(default, Sort::Created)
    }
}
