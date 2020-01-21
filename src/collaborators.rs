use serde_json;

use self::super::{Error, Github};
use crate::users::User;
use http::StatusCode;

use crate::{ErrorKind, Future};
use futures::future::{Future as StdFuture, IntoFuture};
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
        write!(
            f,
            "{}",
            match *self {
                Permissions::Admin => "admin",
                Permissions::Push => "push",
                Permissions::Pull => "pull",
            }
        )
    }
}

pub struct Collaborators {
    github: Github,
    owner: String,
    repo: String,
}

impl Collaborators {
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Collaborators
    where
        O: Into<String>,
        R: Into<String>,
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

    pub fn list(&self) -> Future<Vec<User>> {
        self.github.get::<Vec<User>>(&self.path(""))
    }

    pub fn is_collaborator(&self, username: &str) -> Future<bool> {
        Box::new(
            self.github
                .get::<()>(&self.path(&format!("/{}", username)))
                .map(|_| true)
                .or_else(|err| match err {
                    Error(
                        ErrorKind::Fault {
                            code: StatusCode::NOT_FOUND,
                            ..
                        },
                        _,
                    ) => Ok(false),
                    Error(ErrorKind::Codec(_), _) => Ok(true),
                    otherwise => Err(otherwise),
                }),
        )
    }

    pub fn add(&self, username: &str, permissions: &Permissions) -> Future<()> {
        let mut permission_params = HashMap::new();
        permission_params.insert("permission", permissions.to_string());

        match serde_json::to_string(&permission_params) {
            Ok(data) => self
                .github
                .put::<()>(&self.path(&format!("/{}", username)), data.into_bytes()),
            Err(e) => Box::new(Err(e.into()).into_future()),
        }
    }

    pub fn remove(&self, username: &str) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}", username)))
    }
}
