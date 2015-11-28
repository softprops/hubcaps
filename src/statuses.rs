//! Statuses interface

use self::super::{Github, Result};
use rep::{Status, StatusReq};
use rustc_serialize::json;

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub enum State {
    pending,
    success,
    error,
    failure,
}

impl Default for State {
    fn default() -> State {
        State::pending
    }
}

/// interface for statuses assocaited with a repository
pub struct Statuses<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> Statuses<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Statuses<'a>
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
    pub fn create(&self, sha: &str, status: &StatusReq) -> Result<Status> {
        let data = json::encode(&status).unwrap();
        let body = try!(self.github.post(&self.path(&format!("/{}", sha)), data.as_bytes()));
        Ok(try!(json::decode::<Status>(&body)))
    }

    /// lists all statuses associated with a given git sha
    pub fn list(&self, sha: &str) -> Result<Vec<Status>> {
        let body = try!(self.github.get(&format!("/repos/{}/{}/commits/{}/statuses",
                                                 self.owner,
                                                 self.repo,
                                                 sha)));
        Ok(try!(json::decode::<Vec<Status>>(&body)))
    }

    /// list the combined statuses for a given git sha
    pub fn combined(&self, sha: &str) -> Result<String> {
        let body = try!(self.github.get(&format!("/repos/{}/{}/commits/{}/status",
                                                 self.owner,
                                                 self.repo,
                                                 sha)));
        Ok(body)
        // Ok(json::decode::<Vec<Status>>(&body).unwrap())
    }


}
