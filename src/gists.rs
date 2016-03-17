//! Gists interface
extern crate serde_json;

use self::super::{Error, Github, Result};
use rep::{Gist, GistFork, GistOptions, GistListOptions};

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

    pub fn list(&self, options: &GistListOptions) -> Result<Vec<Gist>> {
        let mut uri = vec![format!("/users/{}/gists", self.owner)];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Gist>>(&uri.join("?"))
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
            Err(Error::Codec(_)) => Ok(()),
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

    pub fn list(&self, options: &GistListOptions) -> Result<Vec<Gist>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Gist>>(&uri.join("?"))
    }

    pub fn public(&self) -> Result<Vec<Gist>> {
        self.github.get::<Vec<Gist>>(&self.path("/public"))
    }

    pub fn starred(&self) -> Result<Vec<Gist>> {
        self.github.get::<Vec<Gist>>(&self.path("/starred"))
    }

    pub fn create(&self, gist: &GistOptions) -> Result<Gist> {
        let data = try!(serde_json::to_string(&gist));
        self.github.post::<Gist>(&self.path(""), data.as_bytes())
    }

    // todo: edit
}
