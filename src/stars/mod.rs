//! Stars interface

use hyper::client::Connect;
use hyper::StatusCode;
use futures::Future as StdFuture;

use {Github, Future, Error, ErrorKind};

pub struct Stars<C>
where
    C: Clone + Connect,
{
    github: Github<C>,
}

impl<C: Clone + Connect> Stars<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }

    pub fn starred<O, R>(&self, owner: O, repo: R) -> Future<bool>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Box::new(
            self.github
                .get::<()>(&format!("/user/starred/{}/{}", owner.into(), repo.into()))
                .map(|_| true)
                .or_else(|err| match err {
                    Error(ErrorKind::Fault { code: StatusCode::NotFound, .. }, _) => Ok(false),
                    Error(ErrorKind::Codec(_), _) => Ok(true),
                    otherwise => Err(otherwise.into()),
                }),
        )
    }

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

    pub fn unstar<O, R>(&self, owner: O, repo: R) -> Future<()>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github.delete(&format!(
            "/user/starred/{}/{}",
            owner.into(),
            repo.into()
        ))
    }
}