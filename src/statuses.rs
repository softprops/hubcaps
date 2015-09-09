//! Statuses interface

use self::super::Github;
use rep::{Status, StatusReq};
use rustc_serialize::json;
use std::io::Result;

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub enum State {
  pending, success, error, failure
}

/// interface for statuses assocaited with a repository
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

  /// creates a new status for a target sha
  pub fn create(&self, sha: &'static str, status: &StatusReq) -> Result<Status> {
    let data = json::encode(&status).unwrap();
    let body = try!(
      self.github.post(
        &self.path(&format!("/{}", sha)),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Status>(&body).unwrap())
  }

  /// lists all statuses associated with a given git sha
  pub fn list(&self, sha: &'static str) -> Result<Vec<Status>> {
    let body = try!(
      self.github.get(
        &format!(
          "/repos/{}/{}/commits/{}/statuses", self.owner, self.repo, sha
        )
      )
    );
    Ok(json::decode::<Vec<Status>>(&body).unwrap())
  }

  /// list the combined statuses for a given git sha
  pub fn combined(&self, sha: &'static str) -> Result<String> {
    let body = try!(
      self.github.get(
        &format!(
          "/repos/{}/{}/commits/{}/status", self.owner, self.repo, sha
        )
      )
    );
    Ok(body)
//    Ok(json::decode::<Vec<Status>>(&body).unwrap())
  }


}
