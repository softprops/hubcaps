//! Deployments interface

use self::super::Github;
use rustc_serialize::json;
use std::io::Result;

pub struct Deployments<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

impl<'a> Deployments<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Deployments<'a> {
    Deployments { github: github, owner: owner, repo: repo }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/deployments{}", self.owner, self.repo, more)
  }

  pub fn list(&self) -> Result<String> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(body)
  }

  pub fn create(&self, gitref: &str) -> Result<String> {
    let body = try!(
      self.github.post(
        &self.path(""),
        &[]
      )
    );
    Ok(body)
  }
}
