//! Git interface

use self::super::{Github, Result};

/// reference to git operations associated with a github repo
pub struct Git<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Git<'a> {
    #[doc(hidden)]
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Git {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: String) -> String {
        format!("/repos/{}/{}/git{}", self.owner, self.repo, more)
    }

    /// list a git tree of files for this repo at a given sha
    /// https://developer.github.com/v3/git/trees/#get-a-tree
    /// https://developer.github.com/v3/git/trees/#get-a-tree-recursively
    pub fn tree<S>(&self, sha: S, recursive: bool) -> Result<TreeData>
    where
        S: Into<String>,
    {
        self.github.get::<TreeData>(&self.path(format!(
            "/trees/{}?recursive={}",
            sha.into(),
            if recursive { "1" } else { "0" }
        )))
    }

    /// get the blob contents of a given sha
    /// https://developer.github.com/v3/git/blobs/#get-a-blob
    pub fn blob<S>(&self, sha: S) -> Result<Blob>
    where
        S: Into<String>,
    {
        self.github.get::<Blob>(
            &self.path(format!("/blobs/{}", sha.into())),
        )
    }
}


// representations

#[derive(Debug, Deserialize)]
pub struct TreeData {
    pub sha: String,
    pub url: String,
    pub tree: Vec<GitFile>,
    pub truncated: bool,
}

#[derive(Debug, Deserialize)]
pub struct GitFile {
    pub path: String,
    pub mode: String,
    /// typically tree or blob
    #[serde(rename = "type")]
    pub content_type: String,
    /// size will be None for directories
    pub size: Option<usize>,
    pub sha: String,
    /// url will be None for commits
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Blob {
    pub content: String,
    pub encoding: String,
    pub url: String,
    pub sha: String,
    /// sizes will be None for directories
    pub size: Option<usize>,
}
