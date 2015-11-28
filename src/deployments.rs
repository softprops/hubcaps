//! Deployments interface

use self::super::{Github, Result};
use rustc_serialize::json;
use rep::{DeploymentReq, DeploymentStatus, DeploymentStatusReq};

/// Interface for repository deployements
pub struct Deployments<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

/// INterface for deployment statuses
pub struct DeploymentStatuses<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    id: u64,
}

impl<'a> DeploymentStatuses<'a> {
    /// creates a new deployment status
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, id: u64) -> DeploymentStatuses<'a>
        where O: Into<String>,
              R: Into<String>
    {
        DeploymentStatuses {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            id: id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/deployments/{}/statuses{}",
                self.owner,
                self.repo,
                self.id,
                more)
    }

    /// lists all statuses associated with a deployment
    pub fn list(&self) -> Result<Vec<DeploymentStatus>> {
        let body = try!(self.github.get(&self.path("")));
        Ok(try!(json::decode::<Vec<DeploymentStatus>>(&body)))
    }

    /// creates a new deployment status. For convenience, a DeploymentStatusReq.builder
    /// interface is required for building up a request
    pub fn create(&self, status: &DeploymentStatusReq) -> Result<DeploymentStatus> {
        let data = try!(json::encode::<DeploymentStatusReq>(&status));
        let body = try!(self.github.post(&self.path(""), &data.as_bytes()));
        Ok(try!(json::decode::<DeploymentStatus>(&body)))
    }
}

impl<'a> Deployments<'a> {
    /// Create a new deployments instance
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Deployments<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Deployments {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/deployments{}", self.owner, self.repo, more)
    }

    /// lists all deployments for a repository
    pub fn list(&self) -> Result<String> {
        let body = try!(self.github.get(&self.path("")));
        Ok(body)
    }

    /// creates a new deployment for this repository
    pub fn create(&self, dep: &DeploymentReq) -> Result<String> {
        let data = try!(json::encode(&dep));
        let body = try!(self.github.post(&self.path(""), data.as_bytes()));
        Ok(body)
    }

    /// get a reference to the statuses api for a give deployment
    pub fn statuses(&self, id: u64) -> DeploymentStatuses {
        DeploymentStatuses::new(self.github, self.owner.as_ref(), self.repo.as_ref(), id)
    }
}
