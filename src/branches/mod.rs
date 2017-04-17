//! Gists interface
extern crate serde_json;

use self::super::{Iter, MediaType, Github, Result};

fn identity<T>(x: T) -> T {
    x
}

/// reference to gists associated with a github user
pub struct Branches<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Branches<'a> {
    /// create a new instance of branches
    pub fn new<U, R>(github: &'a Github, owner: U, repo: R) -> Self
        where U: Into<String>,
              R: Into<String>
    {
        Branches {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list of branches for this repo
    pub fn list(&self) -> Result<Vec<Branch>> {
        self.github
            .get_media::<Vec<Branch>>(&format!("/repos/{owner}/{repo}/branches",
                                               owner = self.owner,
                                               repo = self.repo),
                                      MediaType::Preview("loki"))
    }

    /// provides an iterator over branches for this repo
    pub fn iter(&self) -> Result<Iter<Vec<Branch>, Branch>> {
        self.github
            .iter_media(format!("/repos/{owner}/{repo}/branches",
                                owner = self.owner,
                                repo = self.repo),
                        identity,
                        MediaType::Preview("loki"))
    }

    /// gets a branch for this repo by name
    pub fn get<B>(&self, branch: B) -> Result<Branch>
        where B: Into<String>
    {
        self.github
            .get_media(&format!("/repos/{owner}/{repo}/branches/{branch}",
                                owner = self.owner,
                                repo = self.repo,
                                branch = branch.into()),
                       MediaType::Preview("loki"))
    }
}


// representations

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
    pub protected: bool,
    pub protection_url: String,
    pub protection: Protection, // todo: pub commit: CommitRef
}

#[derive(Debug, Deserialize)]
pub struct Protection {
    pub enabled: bool,
    pub required_status_checks: StatusChecks,
}

#[derive(Debug, Deserialize)]
pub struct StatusChecks {
    pub enforcement_level: String,
    pub contexts: Vec<String>,
}
