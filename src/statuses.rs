//! Statuses interface

use self::super::{Github, Result};
use rep::{Status, StatusOptions};
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
    pub fn create(&self, sha: &str, status: &StatusOptions) -> Result<Status> {
        let data = try!(json::encode(&status));
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
        self.github.get::<String>(&format!("/repos/{}/{}/commits/{}/status",
                                                 self.owner,
                                                 self.repo,
                                                 sha))
    }


}
