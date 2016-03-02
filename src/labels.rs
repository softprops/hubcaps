//! Labels interface

extern crate serde_json;

use self::super::{Github, Result};
use rep::{Label, LabelOptions};

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

    pub fn create(&self, lab: &LabelOptions) -> Result<Label> {
        let data = try!(serde_json::to_string(&lab));
        self.github.post::<Label>(&self.path(""), data.as_bytes())
    }

    pub fn update(&self, prevname: &str, lab: &LabelOptions) -> Result<Label> {
        let data = try!(serde_json::to_string(&lab));
        self.github.patch::<Label>(&self.path(&format!("/{}", prevname)), data.as_bytes())
    }

    pub fn delete(&self, name: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", name)))
            .map(|_| ())
    }

    pub fn list(&self) -> Result<Vec<Label>> {
        self.github.get::<Vec<Label>>(&self.path(""))
    }
}
