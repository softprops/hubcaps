//! Gists interface

use self::super::{Error, Github, Result};
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
        self.github.get::<Vec<Gist>>(&format!("/users/{}/gists", self.owner))
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
        match self.github
                  .put::<String>(&self.path(&format!("/{}/star", id)), &[])
                  .map(|_| ()) {
            Err(Error::Decoding(_)) => Ok(()),
            otherwise => otherwise,
        }
    }

    pub fn unstar(&self, id: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}/star", id)))
    }

    pub fn fork(&self, id: &str) -> Result<Gist> {
        self.github.post::<Gist>(&self.path(&format!("/{}/forks", id)), &[])
    }

    pub fn forks(&self, id: &str) -> Result<Vec<GistFork>> {
        self.github.get::<Vec<GistFork>>(&self.path(&format!("/{}/forks", id)))
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", id)))
    }

    pub fn get(&self, id: &str) -> Result<Gist> {
        self.github.get::<Gist>(&self.path(&format!("/{}", id)))
    }

    pub fn getrev(&self, id: &str, sha: &str) -> Result<Gist> {
        self.github.get::<Gist>(&self.path(&format!("/{}/{}", id, sha)))
    }

    pub fn list(&self) -> Result<Vec<Gist>> {
        self.github.get::<Vec<Gist>>(&self.path(""))
    }

    pub fn public(&self) -> Result<Vec<Gist>> {
        self.github.get::<Vec<Gist>>(&self.path("/public"))
    }

    pub fn starred(&self) -> Result<Vec<Gist>> {
        self.github.get::<Vec<Gist>>(&self.path("/starred"))
    }

    pub fn create(&self, gist: &GistReq) -> Result<Gist> {
        let data = try!(json::encode(&gist));
        self.github.post::<Gist>(&self.path(""), data.as_bytes())
    }

    // todo: edit
}
