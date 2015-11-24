//! Deploy keys interface
//! This [this document](https://developer.github.com/guides/managing-deploy-keys/) for motivation and use

use self::super::{Github, Result};
use rep::{Key, KeyReq};
use rustc_serialize::json;

pub struct Keys<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> Keys<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Keys<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Keys {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/keys{}", self.owner, self.repo, more)
    }

    pub fn create(&self, key: &KeyReq) -> Result<Key> {
        let data = json::encode::<KeyReq>(key).unwrap();
        let body = try!(self.github.post(&self.path(""), data.as_bytes()));
        Ok(json::decode::<Key>(&body).unwrap())
    }

    pub fn list(&self) -> Result<Vec<Key>> {
        let body = try!(self.github.get(&self.path("")));
        Ok(json::decode::<Vec<Key>>(&body).unwrap())
    }

    pub fn get(&self, id: i64) -> Result<Key> {
        let body = try!(self.github.get(&self.path(&format!("/{}", id))));
        Ok(json::decode::<Key>(&body).unwrap())
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", id)))
            .map(|_| ())
    }
}
