//! Deployments interface

use self::super::Github;
use rustc_serialize::json;
use std::io::Result;
use rep::{Deployment, DeploymentReq, DeploymentStatus, DeploymentStatusReq};

/// Interface for repository deployements
pub struct Deployments<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

/// INterface for deployment statuses
pub struct DeploymentStatuses<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str,
  id: i64
}

impl<'a> DeploymentStatuses<'a> {
  /// creates a new
  pub fn new(
    github: &'a Github<'a>,
    owner: &'static str,
    repo: &'static str, id: i64) -> DeploymentStatuses<'a> {
    DeploymentStatuses {
      github: github,
      owner: owner,
      repo: repo,
      id: id
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/deployments/{}/statuses{}", self.owner, self.repo, self.id, more)
  }

  /// lists all statuses associated with a deployment
  pub fn list(&self) -> Result<Vec<DeploymentStatus>> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Vec<DeploymentStatus>>(&body).unwrap())
  }

  /// creates a new deployment status. For convenience, a DeploymentStatusReq.builder
  /// interface is required for building up a request
  pub fn create(&self, status: &DeploymentStatusReq) -> Result<DeploymentStatus> {
    let data = json::encode::<DeploymentStatusReq>(&status).unwrap();
    let body = try!(
      self.github.post(
        &self.path(""),
        &data.as_bytes()
      )
    );
    Ok(json::decode::<DeploymentStatus>(&body).unwrap())
  }
}

impl<'a> Deployments<'a> {
  /// Create a new deployments instance
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Deployments<'a> {
    Deployments { github: github, owner: owner, repo: repo }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/deployments{}", self.owner, self.repo, more)
  }

  /// lists all deployments for a repository
  pub fn list(&self) -> Result<String> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(body)
  }

  /// creates a new deployment for this repository
  pub fn create(&self, dep: &DeploymentReq) -> Result<String> {
    let data = json::encode(&dep).unwrap();
    let body = try!(
      self.github.post(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(body)
  }

  /// get a reference to the statuses api for a give deployment
  pub fn statuses(&self, id: i64) -> DeploymentStatuses {
    DeploymentStatuses::new(self.github, self.owner, self.repo, id)
  }
}
