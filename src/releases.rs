//! Releases inteface

use self::super::{Github, Result};
use rustc_serialize::json;
use rep::{Asset, Release, ReleaseReq};


pub struct Assets<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str,
  releaseid: i64
}


impl<'a> Assets<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str, releaseid: i64) -> Assets<'a> {
    Assets {
      github: github,
      owner: owner,
      repo: repo,
      releaseid: releaseid,
    }
  }

  // todo: upload asset
  // todo: edit asset

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/releases/{}/assets{}", self.owner, self.repo, self.releaseid, more)
  }

  // todo: stream interface to download
  pub fn get(&self, id: i64) -> Result<Asset> {
    let body = try!(
      self.github.get(
        &self.path(
          &format!("/{}", id)
        )
      )
    );
    Ok(json::decode::<Asset>(&body).unwrap())
  }

  pub fn delete(&self, id: i64) -> Result<()> {
    self.github.delete(
      &self.path(
        &format!("/{}", id)
      )
    ).map(|_| ())
  }

  pub fn list(&self) -> Result<Vec<Asset>> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Vec<Asset>>(&body).unwrap())
  }
}

pub struct ReleaseRef<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str,
  id: i64
}

impl<'a> ReleaseRef<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str, id: i64) -> ReleaseRef<'a> {
    ReleaseRef {
      github: github,
      owner: owner,
      repo: repo,
      id: id
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/releases/{}{}", self.owner, self.repo, self.id, more)
  }

  pub fn get(&self) -> Result<Release> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Release>(&body).unwrap())
  }

  pub fn assets(&self) -> Assets {
    Assets::new(self.github, self.owner, self.repo, self.id)
  }
}


pub struct Releases<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

impl<'a> Releases<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Releases<'a> {
    Releases {
      github: github,
      owner: owner,
      repo: repo
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/releases{}", self.owner, self.repo, more)
  }

  pub fn create(&self, rel: &ReleaseReq) -> Result<Release> {
    let data = json::encode(&rel).unwrap();
    let body = try!(
      self.github.post(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Release>(&body).unwrap())
  }

  pub fn edit(&self, id: i64, rel: &ReleaseReq) -> Result<Release> {
    let data = json::encode(&rel).unwrap();
    let body = try!(
      self.github.patch(
        &self.path(
          &format!("/{}", id)
        ),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Release>(&body).unwrap())
  }

  pub fn delete(&self, id: i64) -> Result<()> {
    self.github.delete(
      &self.path(
        &format!("/{}", id)
      )
    ).map(|_| ())
  }

  pub fn list(&self) -> Result<Vec<Release>> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Vec<Release>>(&body).unwrap())
  }

  pub fn get(&self, id: i64) -> ReleaseRef {
    ReleaseRef::new(self.github, self.owner, self.repo, id)
  }
}
