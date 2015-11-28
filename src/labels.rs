//! Labels interface

use self::super::{Github, Result};
use rep::{Label, LabelReq};
use rustc_serialize::json;

pub struct Labels<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> Labels<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Labels<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Labels {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/labels{}", self.owner, self.repo, more)
    }

    pub fn create(&self, lab: &LabelReq) -> Result<Label> {
        let data = try!(json::encode(&lab));
        let body = try!(self.github.post(&self.path(""), data.as_bytes()));
        Ok(try!(json::decode::<Label>(&body)))
    }

    pub fn update(&self, prevname: &str, lab: &LabelReq) -> Result<Label> {
        let data = try!(json::encode(&lab));
        let body = try!(self.github.patch(&self.path(&format!("/{}", prevname)), data.as_bytes()));
        Ok(try!(json::decode::<Label>(&body)))
    }

    pub fn delete(&self, name: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", name)))
            .map(|_| ())
    }

    pub fn list(&self) -> Result<Vec<Label>> {
        let body = try!(self.github.get(&self.path("")));
        Ok(try!(json::decode::<Vec<Label>>(&body)))
    }
}
