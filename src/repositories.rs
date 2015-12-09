//! Repository interface

use self::super::{Github, Result};
use deployments::Deployments;
use keys::Keys;
use issues::{IssueRef, Issues};
use labels::Labels;
use pulls::PullRequests;
use releases::Releases;
use rep::Repo;
use statuses::Statuses;

pub struct Repositories<'a> {
    github: &'a Github<'a>,
}

impl<'a> Repositories<'a> {
    pub fn new(github: &'a Github<'a>) -> Repositories<'a> {
        Repositories { github: github }
    }

    fn path(&self, more: &str) -> String {
        format!("/user/repos{}", more)
    }

    /// list the authenticated users repositories
    /// https://developer.github.com/v3/repos/#list-your-repositories
    // todo: params
    pub fn list(&self) -> Result<Vec<Repo>> {
        self.github.get::<Vec<Repo>>(&self.path(""))
    }
}

/// Provides access to the authenticated user's repositories
pub struct UserRepositories<'a> {
    github: &'a Github<'a>,
    owner: String,
}

impl<'a> UserRepositories<'a> {
    pub fn new<O>(github: &'a Github<'a>, owner: O) -> UserRepositories<'a>
        where O: Into<String>
    {
        UserRepositories {
            github: github,
            owner: owner.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/users/{}/repos{}", self.owner, more)
    }

    /// https://developer.github.com/v3/repos/#list-your-repositories
    pub fn list(&self) -> Result<Vec<Repo>> {
        self.github.get::<Vec<Repo>>(&self.path(""))
    }
}

pub struct Repository<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> Repository<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Repository<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Repository {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// get a reference to [deployments](https://developer.github.com/v3/repos/deployments/)
    /// associated with this repository ref
    pub fn deployments(&self) -> Deployments {
        Deployments::new(self.github, self.owner.as_ref(), self.repo.as_ref())
    }

    /// get a reference to a specific github issue associated with this repoistory ref
    pub fn issue(&self, number: u64) -> IssueRef {
        IssueRef::new(self.github, self.owner.as_ref(), self.repo.as_ref(), number)
    }

    /// get a reference to github issues associated with this repoistory ref
    pub fn issues(&self) -> Issues {
        Issues::new(self.github, self.owner.as_ref(), self.repo.as_ref())
    }

    /// get a reference to [deploy keys](https://developer.github.com/v3/repos/keys/)
    /// associated with this repository ref
    pub fn keys(&self) -> Keys {
        Keys::new(self.github, self.owner.as_ref(), self.repo.as_ref())
    }

    /// get a list of labels associated with this repository ref
    pub fn labels(&self) -> Labels {
        Labels::new(self.github, self.owner.as_ref(), self.repo.as_ref())
    }

    /// get a list of [pulls](https://developer.github.com/v3/pulls/)
    /// associated with this repository ref
    pub fn pulls(&self) -> PullRequests {
        PullRequests::new(self.github, self.owner.as_ref(), self.repo.as_ref())
    }

    /// get a reference to [releases](https://developer.github.com/v3/repos/releases/)
    /// associated with this repository ref
    pub fn releases(&self) -> Releases {
        Releases::new(self.github, self.owner.as_ref(), self.repo.as_ref())
    }

    /// get a references to [statuses](https://developer.github.com/v3/repos/statuses/)
    /// associated with this reposoitory ref
    pub fn statuses(&self) -> Statuses {
        Statuses::new(self.github, self.owner.as_ref(), self.repo.as_ref())
    }
}
