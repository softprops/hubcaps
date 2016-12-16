//! Hooks interface
extern crate serde_json;

use self::super::{Github, Result};
use rep::Hook;

/// Interface for repository hooks
pub struct Hooks<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> Hooks<'a> {
    /// Create a new deployments instance
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Hooks<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Hooks {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    pub fn list(&self) -> Result<Vec<Hook>> {
        self.github.get(&format!("/repos/{}/{}/hooks", self.owner, self.repo))
    }
}
