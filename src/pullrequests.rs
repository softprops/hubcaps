use self::super::{Github, SortDirection};
use rep::{Pull, PullEdit, PullReq};
use rustc_serialize::json;
use std::default::Default;
use std::fmt;
use std::io::Result;

pub enum PullSort {
  Created,
  Updated,
  Popularity,
  LongRunning
}

impl fmt::Display for PullSort {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      PullSort::Created     => "created",
      PullSort::Updated     => "updated",
      PullSort::Popularity  => "popularity",
      PullSort::LongRunning => "long-running"
    })
  }
}

impl Default for PullSort {
  fn default() -> PullSort {
    PullSort::Created
  }
}

pub enum PullState {
  Open,
  Closed,
  All
}

impl fmt::Display for PullState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      PullState::Open   => "open",
      PullState::Closed => "closed",
      PullState::All    => "all"
    })
  }
}

impl Default for PullState {
  fn default() -> PullState {
    PullState::Open
  }
}

pub struct PullRequest<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str,
  number: i64
}

impl<'a> PullRequest<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str, number: i64) -> PullRequest<'a> {
    PullRequest { github: github, owner: owner, repo: repo, number: number }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/pulls/{}{}", self.owner, self.repo, self.number, more)
  }

  pub fn get(&self) -> Result<Pull> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Pull>(&body).unwrap())
  }

  /// short hand for editing state = open
  pub fn open(&self) -> Result<Pull> {
    self.edit(&PullEdit::new(None, None, Some("open")))
  }

  /// shorthand for editing state = closed
  pub fn close(&self) -> Result<Pull> {
    self.edit(&PullEdit::new(None, None, Some("closed")))
  }

  pub fn edit(&self, pr: &PullEdit) -> Result<Pull> {
    let data = json::encode(&pr).unwrap();
    let body = try!(
      self.github.patch(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Pull>(&body).unwrap())
  }
}

pub struct PullRequests<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

impl<'a> PullRequests<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> PullRequests<'a> {
    PullRequests { github: github, owner: owner, repo: repo }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/pulls{}", self.owner, self.repo, more)
  }

  pub fn get(&self, number: i64) -> PullRequest {
    PullRequest::new(self.github, self.owner, self.repo, number)
  }

  pub fn create(&self, pr: &PullReq) -> Result<Pull> {
    let data = json::encode(&pr).unwrap();
    let body = try!(
      self.github.post(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Pull>(&body).unwrap())
  }

  pub fn list(
    &self, state: PullState, sort: PullSort, direction: SortDirection) -> Result<Vec<Pull>> {
    let body = try!(
      self.github.get(
        &self.path(
          &format!(
            "?state={}&sort={}&direction={}", state, sort, direction
          )[..]
        )
      )
    );
    Ok(json::decode::<Vec<Pull>>(&body).unwrap())
  }
}
