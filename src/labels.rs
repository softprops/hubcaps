use self::super::Github;
use rep::{Label, LabelReq};
use rustc_serialize::json;
use std::io::Result;

pub struct Labels<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str,
}

impl<'a> Labels<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> Labels<'a> {
    Labels {
      github: github,
      owner: owner,
      repo: repo
    }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/labels{}", self.owner, self.repo, more)
  }

  pub fn create(&self, lab: &LabelReq) -> Result<Label> {
    let data = json::encode(&lab).unwrap();
    let body = try!(
      self.github.post(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Label>(&body).unwrap())
  }

  pub fn update(&self, prevname: &'static str, lab: &LabelReq) -> Result<Label> {
    let data = json::encode(&lab).unwrap();
    let body = try!(
      self.github.patch(
        &self.path(
          &format!("/{}", prevname)
        ),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Label>(&body).unwrap())
  }

  pub fn delete(&self, name: &'static str) -> Result<()> {
    self.github.delete(
      &self.path(
        &format!("/{}", name)
      )
    ).map(|_| ())
  }

  pub fn list(&self) -> Result<Vec<Label>> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Vec<Label>>(&body).unwrap())
  }
}
