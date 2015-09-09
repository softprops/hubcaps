//! Deploy keys interface

use std::io::Result;
use self::super::Github;

pub struct Keys<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

impl<'a> Keys<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Keys<'a> {
    Keys {
      github: github,
      owner: owner,
      repo: repo
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/keys{}", self.owner, self.repo, more)
  }

  pub fn list(&self) -> Result<String> {
    self.github.get(
      &self.path("")
    )
  }

  pub fn get(&self, id: i64) -> Result<String> {
    self.github.get(
      &self.path(&format!("/{}", id))
    )
  }

  pub fn delete(&self, id: i64) -> Result<()> {
    self.github.delete(
      &self.path(&format!("/{}", id))
      ).map(|_| ())
  }
}
