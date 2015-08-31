use std::io::Result;
use self::super::{Github, SortDirection, State};
use rep::{Issue, IssueReq, Label};
use rustc_serialize::json;
use std::fmt;
use std::default::Default;

pub enum Filter {
  Assigned,
  Created,
  Mentioned,
  Subscribed,
  All
}

impl fmt::Display for Filter {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      Filter::Assigned => "assigned",
      Filter::Created => "created",
      Filter::Mentioned => "mentioned",
      Filter::Subscribed => "subscribed",
      Filter::All => "all"
    })
  }
}

impl Default for Filter {
  fn default() -> Filter {
    Filter::Assigned
  }
}

pub enum Sort {
  Created,
  Updated,
  Comments
}

impl fmt::Display for Sort {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      Sort::Created => "created",
      Sort::Updated => "updated",
      Sort::Comments => "comments"
    })
  }
}

impl Default for Sort {
  fn default() -> Sort {
    Sort::Created
  }
}

pub struct IssueRef<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str,
  number: &'static i64
}

impl<'a> IssueRef<'a> {
  /// create a new instance of a github repo issue ref
  pub fn new(
    github: &'a Github<'a>, owner: &'static str, repo: &'static str,
    number: &'static i64) -> IssueRef<'a> {
    IssueRef {
      github: github,
      owner: owner,
      repo: repo,
      number: number
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/issues/{}{}", self.owner, self.repo, self.number, more)
  }

  // add a set of labels to this issue ref
  pub fn label(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
    let body = try!(self.github.post(
      &self.path("/labels"),
      json::encode(&labels).unwrap().as_bytes()
    ));
    Ok(json::decode::<Vec<Label>>(&body).unwrap())
  }
}

pub struct Issues<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

impl<'a> Issues<'a> {
  /// create a new instance of a github repo issue ref
  pub fn new(
    github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Issues<'a> {
    Issues {
      github: github,
      owner: owner,
      repo: repo
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/issues{}", self.owner, self.repo, more)
  }

  pub fn get(&self, number: &'static i64) -> IssueRef {
    IssueRef::new(self.github, self.owner, self.repo, number)
  }

  pub fn create(&self, is: &IssueReq) -> Result<Issue> {
    let data = json::encode(&is).unwrap();
    let body = try!(
      self.github.post(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Issue>(&body).unwrap())
  }

  pub fn list(
    &self,
    state: State,
    sort: Sort,
    direction: SortDirection,
    assignee: Option<&'static str>,
    creator: Option<&'static str>,
    mentioned: Option<&'static str>,
    labels: Vec<&'static str>,
    since: Option<String>
   ) -> Result<Vec<Issue>> {
    let mut params = Vec::new();
    params.push(format!("state={}", state));
    params.push(format!("sort={}", sort));
    params.push(format!("direction={}", direction));
    if let Some(a) = assignee {
      params.push(format!("assignee={}", a));
    }
    if let Some(c) = creator {
      params.push(format!("creator={}", c));
    }
    let url = self.path(
      &format!("?{}", params.join("&"))
    );
    let body = try!(
      self.github.get(
        &url
      )
    );
    Ok(json::decode::<Vec<Issue>>(&body).unwrap())
  }
}
