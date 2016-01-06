//! Pull requests interface

use url::form_urlencoded;
use self::super::{Github, Result, SortDirection, State};
use rep::{Pull, PullEditOptions, PullOptions};
use rustc_serialize::json;
use std::default::Default;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Sort {
    Created,
    Updated,
    Popularity,
    LongRunning,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Sort::Created => "created",
                   Sort::Updated => "updated",
                   Sort::Popularity => "popularity",
                   Sort::LongRunning => "long-running",
               })
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

pub struct PullRequest<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> PullRequest<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, number: u64) -> PullRequest<'a>
        where O: Into<String>,
              R: Into<String>
    {
        PullRequest {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/pulls/{}{}",
                self.owner,
                self.repo,
                self.number,
                more)
    }

    pub fn get(&self) -> Result<Pull> {
        self.github.get::<Pull>(&self.path(""))
    }

    /// short hand for editing state = open
    pub fn open(&self) -> Result<Pull> {
        self.edit(&PullEditOptions::builder().state("open").build())
    }

    /// shorthand for editing state = closed
    pub fn close(&self) -> Result<Pull> {
        self.edit(&PullEditOptions::builder().state("closed").build())
    }

    pub fn edit(&self, pr: &PullEditOptions) -> Result<Pull> {
        let data = try!(json::encode(&pr));
        self.github.patch::<Pull>(&self.path(""), data.as_bytes())
    }
}

pub struct PullRequests<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

pub struct ListBuilder<'a> {
    pulls: &'a PullRequests<'a>,
    state: State,
    sort: Sort,
    direction: SortDirection,
}

impl<'a> ListBuilder<'a> {
    pub fn new(pulls: &'a PullRequests<'a>) -> ListBuilder<'a> {
        ListBuilder {
            pulls: pulls,
            state: Default::default(),
            sort: Default::default(),
            direction: Default::default(),
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

    pub fn get(&self) -> Result<Vec<Pull>> {
        self.pulls.github.get::<Vec<Pull>>(&self.pulls.path(&format!(
            "?{}", form_urlencoded::serialize(
              vec![
                ("state", self.state.to_string()),
                ("sort", self.sort.to_string()),
                ("direction", self.direction.to_string())
              ]
            )
          )))
    }
}

impl<'a> PullRequests<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> PullRequests<'a>
        where O: Into<String>,
              R: Into<String>
    {
        PullRequests {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/pulls{}", self.owner, self.repo, more)
    }

    pub fn get(&self, number: u64) -> PullRequest {
        PullRequest::new(self.github, self.owner.as_ref(), self.repo.as_ref(), number)
    }

    pub fn create(&self, pr: &PullOptions) -> Result<Pull> {
        let data = try!(json::encode(&pr));
        self.github.post::<Pull>(&self.path(""), data.as_bytes())
    }

    pub fn list(&self) -> ListBuilder {
        ListBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_sort() {
        let default: Sort = Default::default();
        assert_eq!(default, Sort::Created)
    }
}
