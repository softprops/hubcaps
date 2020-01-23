//! Stars interface
use http::StatusCode;

use crate::{Error, ErrorKind, Github, Result};

pub struct Stars {
    github: Github,
}

impl Stars {
    #[doc(hidden)]
    pub fn new(github: Github) -> Self {
        Self { github }
    }

    /// Returns whether or not authenticated user has started a repo
    pub async fn is_starred<O, R>(&self, owner: O, repo: R) -> Result<bool>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .get::<()>(&format!("/user/starred/{}/{}", owner.into(), repo.into()))
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

    /// star a repo
    pub async fn star<O, R>(&self, owner: O, repo: R) -> Result<()>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .put_no_response(
                &format!("/user/starred/{}/{}", owner.into(), repo.into()),
                Vec::new(),
            )
            .await
    }

    /// unstar a repo
    pub async fn unstar<O, R>(&self, owner: O, repo: R) -> Result<()>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .delete(&format!("/user/starred/{}/{}", owner.into(), repo.into()))
            .await
    }
}
