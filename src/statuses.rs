//! Statuses interface
extern crate serde_json;
extern crate serde;

use self::super::{Github, Result};
use rep::{Status, StatusOptions};

/// interface for statuses assocaited with a repository
pub struct Statuses<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Statuses<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Statuses<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Statuses {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/statuses{}", self.owner, self.repo, more)
    }

    /// creates a new status for a target sha
    pub fn create(&self, sha: &str, status: &StatusOptions) -> Result<Status> {
        let data = try!(serde_json::to_string(&status));
        self.github.post::<Status>(&self.path(&format!("/{}", sha)), data.as_bytes())
    }

    /// lists all statuses associated with a given git sha
    pub fn list(&self, sha: &str) -> Result<Vec<Status>> {
        self.github.get::<Vec<Status>>(&format!("/repos/{}/{}/commits/{}/statuses",
                                                self.owner,
                                                self.repo,
                                                sha))
    }

    /// list the combined statuses for a given git sha
    pub fn combined(&self, sha: &str) -> Result<String> {
        self.github
            .get::<String>(&format!("/repos/{}/{}/commits/{}/status", self.owner, self.repo, sha))
    }
}
