//! Releases inteface

use self::super::{Github, Result};
use rustc_serialize::json;
use rep::{Asset, Release, ReleaseReq};


pub struct Assets<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    releaseid: u64,
}


impl<'a> Assets<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, releaseid: u64) -> Assets<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Assets {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            releaseid: releaseid,
        }
    }

    // todo: upload asset
    // todo: edit asset

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/releases/{}/assets{}",
                self.owner,
                self.repo,
                self.releaseid,
                more)
    }

    // todo: stream interface to download
    pub fn get(&self, id: u64) -> Result<Asset> {
        let body = try!(self.github.get(&self.path(&format!("/{}", id))));
        Ok(json::decode::<Asset>(&body).unwrap())
    }

    pub fn delete(&self, id: u64) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", id)))
            .map(|_| ())
    }

    pub fn list(&self) -> Result<Vec<Asset>> {
        let body = try!(self.github.get(&self.path("")));
        Ok(json::decode::<Vec<Asset>>(&body).unwrap())
    }
}

pub struct ReleaseRef<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    id: u64,
}

impl<'a> ReleaseRef<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, id: u64) -> ReleaseRef<'a>
        where O: Into<String>,
              R: Into<String>
    {
        ReleaseRef {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            id: id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/releases/{}{}",
                self.owner,
                self.repo,
                self.id,
                more)
    }

    pub fn get(&self) -> Result<Release> {
        let body = try!(self.github.get(&self.path("")));
        Ok(json::decode::<Release>(&body).unwrap())
    }

    pub fn assets(&self) -> Assets {
        Assets::new(self.github,
                    self.owner.as_ref(),
                    self.repo.as_ref(),
                    self.id)
    }
}


pub struct Releases<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> Releases<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Releases<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Releases {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/releases{}", self.owner, self.repo, more)
    }

    pub fn create(&self, rel: &ReleaseReq) -> Result<Release> {
        let data = json::encode(&rel).unwrap();
        let body = try!(self.github.post(&self.path(""), data.as_bytes()));
        Ok(json::decode::<Release>(&body).unwrap())
    }

    pub fn edit(&self, id: u64, rel: &ReleaseReq) -> Result<Release> {
        let data = json::encode(&rel).unwrap();
        let body = try!(self.github.patch(&self.path(&format!("/{}", id)), data.as_bytes()));
        Ok(json::decode::<Release>(&body).unwrap())
    }

    pub fn delete(&self, id: u64) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", id)))
            .map(|_| ())
    }

    pub fn list(&self) -> Result<Vec<Release>> {
        let body = try!(self.github.get(&self.path("")));
        Ok(json::decode::<Vec<Release>>(&body).unwrap())
    }

    pub fn get(&self, id: u64) -> ReleaseRef {
        ReleaseRef::new(self.github, self.owner.as_ref(), self.repo.as_ref(), id)
    }
}
