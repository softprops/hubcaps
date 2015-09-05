//! Statuses interface

use self::super::Github;
use rep::{Status, StatusReq};
use rustc_serialize::json;
use std::io::Result;

pub struct Statuses<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

impl<'a> Statuses<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Statuses<'a> {
    Statuses {
      github: github,
      owner: owner,
      repo: repo
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/statuses{}", self.owner, self.repo, more)
  }

  fn create(&self, sha: &'static str, status: &StatusReq) -> Result<Status> {
    let data = json::encode(&status).unwrap();
    let body = try!(
      self.github.post(
        &self.path(&format!("/{}", sha)),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Status>(&body).unwrap())
  }

}
