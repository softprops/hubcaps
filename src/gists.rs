//! Gists interface

use self::super::{Github, Result};
use rustc_serialize::json;
use rep::{Gist, GistFork, GistReq};

/// reference to gists associated with a github user
pub struct UserGists<'a> {
    github: &'a Github<'a>,
    owner: String,
}

impl<'a> UserGists<'a> {

    pub fn new<O>(github: &'a Github<'a>, owner: O) -> UserGists<'a>
        where O: Into<String>
    {
        UserGists {
            github: github,
            owner: owner.into(),
        }
    }

    pub fn list(&self) -> Result<Vec<Gist>> {
        let body = try!(self.github.get(&format!("/users/{}/gists", self.owner)));
        Ok(try!(json::decode::<Vec<Gist>>(&body)))
    }
}

pub struct Gists<'a> {
    github: &'a Github<'a>,
}

impl<'a> Gists<'a> {
    pub fn new(github: &'a Github<'a>) -> Gists<'a> {
        Gists { github: github }
    }

    fn path(&self, more: &str) -> String {
        format!("/gists{}", more)
    }

    pub fn star(&self, id: &str) -> Result<()> {
        self.github
            .put(&self.path(&format!("/{}/star", id)), &[])
            .map(|_| ())
    }

    pub fn unstar(&self, id: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}/star", id)))
            .map(|_| ())
    }

    pub fn fork(&self, id: &str) -> Result<Gist> {
        let body = try!(self.github.post(&self.path(&format!("/{}/forks", id)), &[]));
        Ok(try!(json::decode::<Gist>(&body)))
    }

    pub fn forks(&self, id: &str) -> Result<Vec<GistFork>> {
        let body = try!(self.github.get(&self.path(&format!("/{}/forks", id))));
        Ok(try!(json::decode::<Vec<GistFork>>(&body)))
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", id)))
            .map(|_| ())
    }

    pub fn get(&self, id: &str) -> Result<Gist> {
        let body = try!(self.github.get(&self.path(&format!("/{}", id))));
        Ok(try!(json::decode::<Gist>(&body)))
    }

    pub fn getrev(&self, id: &str, sha: &str) -> Result<Gist> {
        let body = try!(self.github.get(&self.path(&format!("/{}/{}", id, sha))));
        Ok(try!(json::decode::<Gist>(&body)))
    }

    pub fn list(&self) -> Result<Vec<Gist>> {
        let body = try!(self.github.get(&self.path("")));
        Ok(try!(json::decode::<Vec<Gist>>(&body)))
    }

    pub fn public(&self) -> Result<Vec<Gist>> {
        let body = try!(self.github.get(&self.path("/public")));
        Ok(try!(json::decode::<Vec<Gist>>(&body)))
    }

    pub fn starred(&self) -> Result<Vec<Gist>> {
        let body = try!(self.github.get(&self.path("/starred")));
        Ok(try!(json::decode::<Vec<Gist>>(&body)))
    }

    pub fn create(&self, gist: &GistReq) -> Result<Gist> {
        let data = json::encode(&gist).unwrap();
        let body = try!(self.github.post(&self.path(""), data.as_bytes()));
        Ok(try!(json::decode::<Gist>(&body)))
    }
}
