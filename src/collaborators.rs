extern crate serde_json;

use rep::User;
use self::super::{Github, Result, Error};
use hyper::status::StatusCode;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Permissions {
    Admin,
    Push,
    Pull,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Permissions::Push
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Permissions::Admin => "admin",
                   Permissions::Push => "push",
                   Permissions::Pull => "pull",
               }
        )
    }
}

pub struct Collaborators<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

impl<'a> Collaborators<'a> {
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Collaborators<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Collaborators {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/collaborators{}", self.owner, self.repo, more)
    }

    pub fn list(&self) -> Result<Vec<User>> {
        self.github.get::<Vec<User>>(&self.path(""))
    }

    pub fn is_collaborator(&self, username: &str) -> Result<bool> {
        match self.github.get::<()>(&self.path(&format!("/{}", username))) {
            Ok(_) => Ok(true),
            Err(Error::Fault { code: c, .. }) if c == StatusCode::NotFound => Ok(false),
            Err(other) => Err(other),
        }
    }

    pub fn add(&self, username: &str, permissions: &Permissions) -> Result<()> {
        let mut permission_params = HashMap::new();
        permission_params.insert("permission", permissions.to_string());
        let data = try!(serde_json::to_string(&permission_params));

        self.github.put::<()>(
            &self.path(&format!("/{}", username)),
            data.as_bytes()
        )
    }

    pub fn remove(&self, username: &str) -> Result<()> {
        self.github.delete(&self.path(&format!("/{}", username)))
    }

}
