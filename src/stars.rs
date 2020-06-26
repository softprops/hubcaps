//! Stars interface
use futures::prelude::*;
use http::StatusCode;

use crate::{Error, Future, Github};

pub struct Stars {
    github: Github,
}

impl Stars {
    #[doc(hidden)]
    pub fn new(github: Github) -> Self {
        Self { github }
    }

    /// Returns whether or not authenticated user has started a repo
    pub fn is_starred<O, R>(&self, owner: O, repo: R) -> Future<bool>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Box::pin(
            self.github
                .get::<()>(&format!("/user/starred/{}/{}", owner.into(), repo.into()))
                .map_ok(|_| true)
                .or_else(|err| async move {
                    match err {
                        Error::Fault {
                            code: StatusCode::NOT_FOUND,
                            ..
                        } => Ok(false),
                        Error::Codec(_) => Ok(true),
                        otherwise => Err(otherwise),
                    }
                }),
        )
    }

    /// star a repo
    pub fn star<O, R>(&self, owner: O, repo: R) -> Future<()>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github.put_no_response(
            &format!("/user/starred/{}/{}", owner.into(), repo.into()),
            Vec::new(),
        )
    }

    /// unstar a repo
    pub fn unstar<O, R>(&self, owner: O, repo: R) -> Future<()>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .delete(&format!("/user/starred/{}/{}", owner.into(), repo.into()))
    }
}
