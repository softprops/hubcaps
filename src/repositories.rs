//! Repository interface

use self::super::Github;
use deployments::Deployments;
use keys::Keys;
use issues::{IssueRef, Issues};
use labels::Labels;
use pullrequests::PullRequests;
use releases::Releases;
use statuses::Statuses;

pub struct Repositories<'a> {
    github: &'a Github<'a>
}

impl <'a> Repositories<'a> {
    pub fn new(
        github: &'a Github<'a>) -> Repositories<'a> {
        Repositories {
            github: github
        }
    }
}

pub struct UserRepositories<'a> {
    github: &'a Github<'a>,
    owner: &'static str
}

impl <'a> UserRepositories<'a> {
    pub fn new(
        github: &'a Github<'a>,
        owner: &'static str) -> UserRepositories<'a> {
        UserRepositories {
            github: github,
            owner: owner
        }
    }
}

pub struct Repository<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

impl<'a> Repository<'a> {
  pub fn new(
    github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Repository<'a> {
    Repository {
      github: github,
      owner: owner,
      repo: repo
    }
  }

  /// get a reference to deployments associated with this repository ref
  pub fn deployments(&self) -> Deployments {
    Deployments::new(self.github, self.owner, self.repo)
  }

  /// get a reference to a specific github issue associated with this repoistory ref
  pub fn issue(&self, number: i64) -> IssueRef {
    IssueRef::new(self.github, self.owner, self.repo, number)
  }

  /// get a reference to github issues associated with this repoistory ref
  pub fn issues(&self) -> Issues {
    Issues::new(self.github, self.owner, self.repo)
  }

  /// get a reference deploy keys associated with this repository ref
  pub fn keys(&self) -> Keys {
    Keys::new(self.github, self.owner, self.repo)
  }

  /// get a list of labels associated with this repository ref
  pub fn labels(&self) -> Labels {
    Labels::new(self.github, self.owner, self.repo)
  }

  /// get a list of pulls associated with this repository ref
  pub fn pulls(&self) -> PullRequests {
    PullRequests::new(self.github, self.owner, self.repo)
  }

  /// get a reference to releases associated with this repository ref
  pub fn releases(&self) -> Releases {
    Releases::new(self.github, self.owner, self.repo)
  }

  /// get a references to statuses associated with this reposoitory ref
  pub fn statuses(&self) -> Statuses {
    Statuses::new(self.github, self.owner, self.repo)
  }
}
