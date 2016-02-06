//! Statuses interface

use self::super::{Github, Result};
use rep::{Status, StatusOptions};
use rustc_serialize::{json, Decoder, Decodable, Encodable, Encoder};
use std::result::Result as StdResult;

#[derive(Clone, Debug)]
pub enum State {
    Pending,
    Success,
    Error,
    Failure,
}

impl Decodable for State {
    fn decode<D: Decoder>(decoder: &mut D) -> StdResult<State, D::Error> {
        decoder.read_enum("State", |d| {
            d.read_enum_variant(&["pending", "success", "error", "failure"], |_, i| {
                match i {
                    0 => Ok(State::Pending),
                    1 => Ok(State::Success),
                    2 => Ok(State::Error),
                    3 => Ok(State::Failure),
                    _ => unreachable!(),
                }
            })
        })

    }
}

impl Encodable for State {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> StdResult<(), E::Error> {
        try!(match *self {
            State::Pending => {
                encoder.emit_enum("State",
                                  |e| e.emit_enum_variant("pending", 0, 0, |e| "pending".encode(e)))
            }
            State::Success => {
                encoder.emit_enum("State",
                                  |e| e.emit_enum_variant("success", 1, 0, |e| "success".encode(e)))
            }
            State::Error => {
                encoder.emit_enum("State",
                                  |e| e.emit_enum_variant("error", 2, 0, |e| "error".encode(e)))
            }
            State::Failure => {
                encoder.emit_enum("State",
                                  |e| e.emit_enum_variant("failure", 3, 0, |e| "failure".encode(e)))
            }
        });
        Ok(())
    }
}

impl Default for State {
    fn default() -> State {
        State::Pending
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
        self.github
            .get::<String>(&format!("/repos/{}/{}/commits/{}/status", self.owner, self.repo, sha))
    }
}
