use serde_json;

use self::super::{Error, Github};
use crate::users::User;
use http::StatusCode;

use crate::{ErrorKind, Future, Result};
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

    pub async fn list(&self) -> Result<Vec<User>> {
        self.github.get::<Vec<User>>(&self.path("")).await
    }

    pub async fn is_collaborator(&self, username: &str) -> Result<bool> {
        self.github
            .get::<()>(&self.path(&format!("/{}", username)))
            .await
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
            })
    }

    pub async fn add(&self, username: &str, permissions: &Permissions) -> Result<()> {
        let mut permission_params = HashMap::new();
        permission_params.insert("permission", permissions.to_string());

        self.github
            .put::<()>(
                &self.path(&format!("/{}", username)),
                serde_json::to_string(&permission_params)?.into_bytes(),
            )
            .await
    }

    pub async fn remove(&self, username: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", username)))
            .await
    }
}
