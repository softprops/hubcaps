//! Issues interface

extern crate serde_json;

use super::{Github, Result};
use comments::Comments;
use rep::{Issue, IssueOptions, IssueListOptions, Label};
use std::fmt;
use std::default::Default;

#[derive(Debug, PartialEq)]
pub enum Filter {
    Assigned,
    Created,
    Mentioned,
    Subscribed,
    All,
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Filter::Assigned => "assigned",
                   Filter::Created => "created",
                   Filter::Mentioned => "mentioned",
                   Filter::Subscribed => "subscribed",
                   Filter::All => "all",
               })
    }
}

impl Default for Filter {
    fn default() -> Filter {
        Filter::Assigned
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sort {
    Created,
    Updated,
    Comments,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Sort::Created => "created",
                   Sort::Updated => "updated",
                   Sort::Comments => "comments",
               })
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

pub struct IssueLabels<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> IssueLabels<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, number: u64) -> IssueLabels<'a>
        where O: Into<String>,
              R: Into<String>
    {
        IssueLabels {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues/{}/labels{}",
                self.owner,
                self.repo,
                self.number,
                more)
    }

    /// add a set of labels to this issue ref
    pub fn add(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
        self.github
            .post::<Vec<Label>>(&self.path(""),
                                try!(serde_json::to_string(&labels)).as_bytes())
    }

    /// remove a label from this issue
    pub fn remove(&self, label: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", label)))
    }

    /// replace all labels associated with this issue with a new set.
    /// providing an empty set of labels is the same as clearing the
    /// current labels
    pub fn set(&self, labels: Vec<&str>) -> Result<Vec<Label>> {
        self.github.put::<Vec<Label>>(&self.path(""),
                                      try!(serde_json::to_string(&labels)).as_bytes())
    }

    /// remove all labels from an issue
    pub fn clear(&self) -> Result<()> {
        self.github
            .delete(&self.path(""))
    }
}

pub struct IssueRef<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> IssueRef<'a> {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, number: u64) -> IssueRef<'a>
        where O: Into<String>,
              R: Into<String>
    {
        IssueRef {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues/{}{}",
                self.owner,
                self.repo,
                self.number,
                more)
    }

    pub fn labels(&self) -> IssueLabels {
        IssueLabels::new(self.github,
                         self.owner.as_str(),
                         self.repo.as_str(),
                         self.number)
    }

    pub fn edit(&self, is: &IssueOptions) -> Result<Issue> {
        let data = try!(serde_json::to_string(&is));
        self.github.patch::<Issue>(&self.path(""), data.as_bytes())
    }

    pub fn comments(&self) -> Comments {
        Comments::new(self.github,
                      self.owner.clone(),
                      self.repo.clone(),
                      self.number)
    }
}


pub struct Issues<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> Issues<'a> {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Issues<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Issues {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues{}", self.owner, self.repo, more)
    }

    pub fn get(&self, number: u64) -> IssueRef {
        IssueRef::new(self.github, self.owner.as_str(), self.repo.as_str(), number)
    }

    pub fn create(&self, is: &IssueOptions) -> Result<Issue> {
        let data = try!(serde_json::to_string(&is));
        self.github.post::<Issue>(&self.path(""), data.as_bytes())
    }

    pub fn list(&self, options: &IssueListOptions) -> Result<Vec<Issue>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Issue>>(&uri.join("?"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_default() {
        let default: Filter = Default::default();
        assert_eq!(default, Filter::Assigned)
    }

    #[test]
    fn filter_display() {
        for (k, v) in vec![
            (Filter::Assigned, "assigned"),
            (Filter::Created, "created"),
            (Filter::Mentioned, "mentioned"),
            (Filter::Subscribed, "subscribed"),
            (Filter::All, "all"),
        ] {
            assert_eq!(k.to_string(), v)
        }
    }

    #[test]
    fn sort_default() {
        let default: Sort = Default::default();
        assert_eq!(default, Sort::Created)
    }

    #[test]
    fn sort_display() {
        for (k, v) in vec![(Sort::Created, "created"),
                           (Sort::Updated, "updated"),
                           (Sort::Comments, "comments")] {
            assert_eq!(k.to_string(), v)
        }
    }
}
