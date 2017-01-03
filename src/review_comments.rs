
use super::{Github, Result};
use rep::ReviewComment;

/// A structure for interfacing with a issue comments
pub struct ReviewComments<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> ReviewComments<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, number: u64) -> ReviewComments<'a>
        where O: Into<String>,
              R: Into<String>
    {
        ReviewComments {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    /// list pull requests
    pub fn list(&self) -> Result<Vec<ReviewComment>> {
        self.github.get::<Vec<ReviewComment>>(&format!("/repos/{}/{}/pulls/{}/comments",
                                                       self.owner,
                                                       self.repo,
                                                       self.number))
    }
}
