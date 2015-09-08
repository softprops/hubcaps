//! Deployments interface

use self::super::Github;
use rustc_serialize::json;
use std::io::Result;
use rep::{Deployment, DeploymentReq, DeploymentStatus, DeploymentStatusReq};

pub struct Deployments<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

pub struct DeploymentStatuses<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str,
  id: i64
}

impl<'a> DeploymentStatuses<'a> {
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

  pub fn list(&self) -> Result<Vec<DeploymentStatus>> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Vec<DeploymentStatus>>(&body).unwrap())
  }

  pub fn create(&self, stat: &DeploymentStatusReq) -> Result<DeploymentStatus> {
    let data = json::encode::<DeploymentStatusReq>(&stat).unwrap();
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
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Deployments<'a> {
    Deployments { github: github, owner: owner, repo: repo }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/deployments{}", self.owner, self.repo, more)
  }

  pub fn list(&self) -> Result<String> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(body)
  }

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

  pub fn statuses(&self, id: i64) -> DeploymentStatuses {
    DeploymentStatuses::new(self.github, self.owner, self.repo, id)
  }
}
