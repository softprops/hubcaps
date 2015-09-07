//! Issues interface

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

  /// add a set of labels to this issue ref
  pub fn label(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
    let body = try!(self.github.post(
      &self.path("/labels"),
      json::encode(&labels).unwrap().as_bytes()
    ));
    Ok(json::decode::<Vec<Label>>(&body).unwrap())
  }

  /// remove a label from this issue
  pub fn unlabel(&self, label: &'static str) -> Result<()> {
    self.github.delete(
      &self.path(
        &format!("/labels/{}", label)
      )
    ).map(|_| ())
  }

  /// replace all labels associated with this issue with a new set.
  /// providing an empty set of labels is the same as clearing the
  /// current labels
  pub fn relabel(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
    let body = try!(self.github.patch(
      &self.path("/labels"),
      json::encode(&labels).unwrap().as_bytes()
    ));
    Ok(json::decode::<Vec<Label>>(&body).unwrap())
  }

  pub fn edit(&self, is: &IssueReq) -> Result<Issue> {
    let data = json::encode(&is).unwrap();
    let body = try!(
      self.github.patch(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Issue>(&body).unwrap())
  }
}


pub struct Issues<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

/// an mutable issue list builder
pub struct ListBuilder<'a> {
  issues: &'a Issues<'a>,
  state: State,
  sort: Sort,
  direction: SortDirection,
  assignee: Option<&'static str>,
  creator: Option<&'static str>,
  mentioned: Option<&'static str>,
  labels: Vec<&'static str>,
  since: Option<&'static str>
}

impl<'a> ListBuilder<'a> {

  pub fn new(is: &'a Issues<'a>) -> ListBuilder<'a> {
    ListBuilder {
      issues: is,
      state: Default::default(),
      sort: Default::default(),
      direction: Default::default(),
      assignee: None,
      creator: None,
      mentioned: None,
      labels: vec![],
      since: None
    }
  }

  pub fn state(&mut self, state: State) -> &mut ListBuilder<'a> {
    self.state = state;
    self
  }

  pub fn sort(&mut self, sort: Sort) -> &mut ListBuilder<'a> {
    self.sort = sort;
    self
  }

  pub fn direction(&mut self, direction: SortDirection) -> &mut ListBuilder<'a> {
    self.direction = direction;
    self
  }

  pub fn assignee(&mut self, assignee: &'static str) -> &mut ListBuilder<'a> {
    self.assignee = Some(assignee);
    self
  }

  pub fn creator(&mut self, creator: &'static str) -> &mut ListBuilder<'a> {
    self.creator = Some(creator);
    self
  }

  pub fn mentioned(&mut self, mentioned: &'static str) -> &mut ListBuilder<'a> {
    self.mentioned = Some(mentioned);
    self
  }

  pub fn labels(&mut self, labels: Vec<&'static str>) -> &mut ListBuilder<'a> {
    self.labels = labels;
    self
  }

  pub fn since(&mut self, since: &'static str) -> &mut ListBuilder<'a> {
    self.since = Some(since);
    self
  }

  pub fn get(&self) -> Result<Vec<Issue>> {
    let mut params = Vec::new();
    params.push(format!("state={}", self.state));
    params.push(format!("sort={}", self.sort));
    params.push(format!("direction={}", self.direction));
    if let Some(a) = self.assignee {
      params.push(format!("assignee={}", a));
    }
    if let Some(c) = self.creator {
      params.push(format!("creator={}", c));
    }
    if let Some(m) = self.mentioned {
      params.push(format!("mentioned={}", m));
    }
    if let Some(s) = self.since {
      params.push(format!("since={}", s));
    }
    if !self.labels.is_empty() {
      params.push(format!("labels={}", self.labels.connect(",")));
    }
    println!("params {:?}", params);
    let url = self.issues.path(
      &format!("?{}", params.connect("&"))
    );
    let body = try!(
      self.issues.github.get(
        &url
      )
    );
    Ok(json::decode::<Vec<Issue>>(&body).unwrap())
  }

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

  pub fn list(&self) -> ListBuilder {
    ListBuilder::new(self)
  }
}
