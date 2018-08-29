//! Content interface

use hyper::client::connect::Connect;

use {Future, Github};

/// Provides access to the content information for a repository
pub struct Content<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> Content<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Content {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/contents{}", self.owner, self.repo, more)
    }

    /// List the root directory
    pub fn root(&self) -> Future<Vec<DirectoryItem>> {
        self.github.get(&self.path("/"))
    }

    /// Information on a single file
    pub fn file(&self, location: &str) -> Future<File> {
        self.github.get(&self.path(location))
    }

    /// List the files in a directory
    pub fn directory(&self, location: &str) -> Future<Vec<DirectoryItem>> {
        self.github.get(&self.path(location))
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct File {
    #[serde(rename = "type")]
    pub _type: String,
    pub encoding: String,
    pub size: u32,
    pub name: String,
    pub path: String,
    pub content: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    pub _links: Links,
}

#[derive(Debug, Deserialize)]
pub struct DirectoryItem {
    #[serde(rename = "type")]
    pub _type: String,
    pub size: u32,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: Option<String>,
    pub _links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Symlink {
    #[serde(rename = "type")]
    pub _type: String,
    pub target: String,
    pub size: u32,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    pub _links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Submodule {
    #[serde(rename = "type")]
    pub _type: String,
    pub submodule_git_url: String,
    pub size: u32,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: Option<String>,
    pub _links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub git: String,
    #[serde(rename = "self")]
    pub _self: String,
    pub html: String,
}