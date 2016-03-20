use rep::User;
use self::super::{Github, Result, Error};
use hyper::status::StatusCode;

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
}
