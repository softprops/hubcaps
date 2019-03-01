//! Git interface

// Third party
use serde::Deserialize;

// Ours
use crate::{Future, Github};

/// reference to git operations associated with a github repo
pub struct Git {
    github: Github,
    owner: String,
    repo: String,
}

impl Git {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Git {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/git{}", self.owner, self.repo, more)
    }

    /// list a git tree of files for this repo at a given sha
    /// https://developer.github.com/v3/git/trees/#get-a-tree
    /// https://developer.github.com/v3/git/trees/#get-a-tree-recursively
    pub fn tree<S>(&self, sha: S, recursive: bool) -> Future<TreeData>
    where
        S: Into<String>,
    {
        self.github.get(&self.path(&format!(
            "/trees/{}?recursive={}",
            sha.into(),
            if recursive { "1" } else { "0" }
        )))
    }

    /// get the blob contents of a given sha
    /// https://developer.github.com/v3/git/blobs/#get-a-blob
    pub fn blob<S>(&self, sha: S) -> Future<Blob>
    where
        S: Into<String>,
    {
        self.github
            .get(&self.path(&format!("/blobs/{}", sha.into())))
    }

    /// get the git reference data of a given ref
    /// the specified reference must be formatted as as "heads/branch", not just "branch"
    /// https://developer.github.com/v3/git/refs/#get-a-reference
    pub fn reference<S>(&self, reference: S) -> Future<GetReferenceResponse>
    where
        S: Into<String>,
    {
        self.github
            .get(&self.path(&format!("/refs/{}", reference.into())))
    }

    //// deletes a refish
    /// branches should be in the format `heads/feature-a`
    /// tags should be in the format `tags/v1.0`
    /// https://developer.github.com/v3/git/refs/#delete-a-reference
    pub fn delete_reference<S>(&self, reference: S) -> Future<()>
    where
        S: Into<String>,
    {
        self.github
            .delete(&self.path(&format!("/refs/{}", reference.into())))
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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
/// The response for getting a git reference
pub enum GetReferenceResponse {
    /// The reference data matching the specified reference
    Exact(Reference),
    /// If the reference doesn't exist in the repository
    /// but existing refs start with ref they will be returned as an array.
    /// For example, a call to get the data for a branch named feature,
    /// which doesn't exist, would return head refs including featureA and featureB which do.
    StartWith(Vec<Reference>),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Reference {
    #[serde(rename = "ref")]
    pub reference: String,
    pub url: String,
    pub object: Object,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Object {
    #[serde(rename = "type")]
    pub object_type: String,
    pub sha: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json;
    use std::fmt::Debug;

    fn test_deserializing<'de, T>(payload: &'static str, expected: T)
    where
        T: Debug + PartialEq + Deserialize<'de>,
    {
        let incoming: T = serde_json::from_str(payload).unwrap();
        assert_eq!(incoming, expected)
    }

    #[test]
    fn deserialize_get_ref_exact() {
        let payload = r#"{
  "ref": "refs/heads/featureA",
  "url": "https://api.github.com/repos/octocat/Hello-World/git/refs/heads/featureA",
  "object": {
    "type": "commit",
    "sha": "aa218f56b14c9653891f9e74264a383fa43fefbd",
    "url": "https://api.github.com/repos/octocat/Hello-World/git/commits/aa218f56b14c9653891f9e74264a383fa43fefbd"
  }
}"#;
        let expected = GetReferenceResponse::Exact(Reference {
            reference: "refs/heads/featureA".to_string(),
            url: "https://api.github.com/repos/octocat/Hello-World/git/refs/heads/featureA".to_string(),
            object: Object {
                object_type: "commit".to_string(),
                sha: "aa218f56b14c9653891f9e74264a383fa43fefbd".to_string(),
                url: "https://api.github.com/repos/octocat/Hello-World/git/commits/aa218f56b14c9653891f9e74264a383fa43fefbd".to_string(),
            },
        });
        test_deserializing(payload, expected)
    }

    #[test]
    fn deserialize_get_ref_starts_with() {
        let payload = r#"[
  {
    "ref": "refs/heads/feature-a",
    "url": "https://api.github.com/repos/octocat/Hello-World/git/refs/heads/feature-a",
    "object": {
      "type": "commit",
      "sha": "aa218f56b14c9653891f9e74264a383fa43fefbd",
      "url": "https://api.github.com/repos/octocat/Hello-World/git/commits/aa218f56b14c9653891f9e74264a383fa43fefbd"
    }
  },
  {
    "ref": "refs/heads/feature-b",
    "url": "https://api.github.com/repos/octocat/Hello-World/git/refs/heads/feature-b",
    "object": {
      "type": "commit",
      "sha": "612077ae6dffb4d2fbd8ce0cccaa58893b07b5ac",
      "url": "https://api.github.com/repos/octocat/Hello-World/git/commits/612077ae6dffb4d2fbd8ce0cccaa58893b07b5ac"
    }
  }
]"#;
        let expected = GetReferenceResponse::StartWith(vec![
            Reference {
                reference: "refs/heads/feature-a".to_string(),
                url: "https://api.github.com/repos/octocat/Hello-World/git/refs/heads/feature-a".to_string(),
                object: Object {
                    object_type: "commit".to_string(),
                    sha: "aa218f56b14c9653891f9e74264a383fa43fefbd".to_string(),
                    url: "https://api.github.com/repos/octocat/Hello-World/git/commits/aa218f56b14c9653891f9e74264a383fa43fefbd".to_string(),
                },
            },
            Reference {
                reference: "refs/heads/feature-b".to_string(),
                url: "https://api.github.com/repos/octocat/Hello-World/git/refs/heads/feature-b".to_string(),
                object: Object {
                    object_type: "commit".to_string(),
                    sha: "612077ae6dffb4d2fbd8ce0cccaa58893b07b5ac".to_string(),
                    url: "https://api.github.com/repos/octocat/Hello-World/git/commits/612077ae6dffb4d2fbd8ce0cccaa58893b07b5ac".to_string(),
                },
            },
        ]);
        test_deserializing(payload, expected)
    }
}
