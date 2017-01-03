//! Comments interface

use super::{Github, Result};
use rep::{Comment, CommentListOptions};

/// A structure for interfacing with a issue comments
pub struct Comments<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> Comments<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, number: u64) -> Comments<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Comments {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    /// list pull requests
    pub fn list(&self, options: &CommentListOptions) -> Result<Vec<Comment>> {
        let mut uri = vec![format!("/repos/{}/{}/issues/{}/comments",
                                   self.owner,
                                   self.repo,
                                   self.number)];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Comment>>(&uri.join("?"))
    }
}
