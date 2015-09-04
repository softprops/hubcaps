use self::super::Github;
use rep::Label;
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

  pub fn list(&self) -> Result<Vec<Label>> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Vec<Label>>(&body).unwrap())
  }
}
