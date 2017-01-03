//! Hooks interface
extern crate serde_json;

use self::super::{Github, Result};
use rep::{Hook, HookCreateOptions};

/// Interface for mangaing repository hooks
pub struct Hooks<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Hooks<'a> {
    /// Create a new deployments instance
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Hooks<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Hooks {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// lists hook associated with a repoistory
    pub fn list(&self) -> Result<Vec<Hook>> {
        self.github.get(&format!("/repos/{}/{}/hooks", self.owner, self.repo))
    }

    /// creates a new repository hook
    pub fn create(&self, options: &HookCreateOptions) -> Result<Hook> {
        let data = try!(serde_json::to_string(&options));
        self.github.post::<Hook>(&format!("/repos/{}/{}/hooks", self.owner, self.repo),
                                 data.as_bytes())
    }

    /// deletes a repoistory hook by id
    pub fn delete(&self, id: u64) -> Result<()> {
        self.github.delete(&format!("/repos/{}/{}/hooks/{}", self.owner, self.repo, id))
    }
}
