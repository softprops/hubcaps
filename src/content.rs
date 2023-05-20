//! Content interface
use std::fmt;
use std::ops;

use data_encoding::BASE64;
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};

use crate::repo_commits::CommitDetails;
use crate::utils::{percent_encode, PATH};
use crate::{Future, Github, Stream};

/// Provides access to the content information for a repository
pub struct Content {
    github: Github,
    owner: String,
    repo: String,
}

impl Content {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
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

    fn path(&self, location: &str, ref_: &str) -> String {
        // Handle files with spaces and other characters that can mess up the
        // final URL.
        let location = percent_encode(location.as_ref(), PATH);
        let mut path = format!("/repos/{}/{}/contents{}", self.owner, self.repo, location);
        if !ref_.is_empty() {
            path += &format!("?ref={}", ref_);
        }
        path
    }

    /// Gets the contents of the location. This could be a file, symlink, or
    /// submodule. To list the contents of a directory, use `iter`.
    pub fn get(&self, location: &str, ref_: &str) -> Future<Contents> {
        self.github.get(&self.path(location, ref_))
    }

    /// Information on a single file.
    ///
    /// GitHub only supports downloading files up to 1 megabyte in size. If you
    /// need to retrieve larger files, the Git Data API must be used instead.
    pub fn file(&self, location: &str, ref_: &str) -> Future<File> {
        self.github.get(&self.path(location, ref_))
    }

    /// List the root directory.
    pub fn root(&self, ref_: &str) -> Stream<DirectoryItem> {
        self.iter("/", ref_)
    }

    /// Provides a stream over the directory items in `location`.
    ///
    /// GitHub limits the number of items returned to 1000 for this API. If you
    /// need to retrieve more items, the Git Data API must be used instead.
    pub fn iter(&self, location: &str, ref_: &str) -> Stream<DirectoryItem> {
        self.github.get_stream(&self.path(location, ref_))
    }

    /// Creates a file at a specific location in a repository.
    /// You DO NOT need to base64 encode the content, we will do it for you.
    pub fn create(&self, location: &str, content: &[u8], message: &str) -> Future<NewFileResponse> {
        let file = &NewFile {
            content: BASE64.encode(content),
            message: message.to_string(),
            sha: None,
        };
        self.github.put(&self.path(location, ""), json!(file))
    }

    /// Updates a file at a specific location in a repository.
    /// You DO NOT need to base64 encode the content, we will do it for you.
    pub fn update(
        &self,
        location: &str,
        content: &[u8],
        message: &str,
        sha: &str,
    ) -> Future<NewFileResponse> {
        let file = &NewFile {
            content: BASE64.encode(content),
            message: message.to_string(),
            sha: Some(sha.to_string()),
        };
        self.github.put(&self.path(location, ""), json!(file))
    }

    /// Deletes a file.
    pub fn delete(&self, location: &str, message: &str, sha: &str) -> Future<()> {
        let file = &NewFile {
            content: "".to_string(),
            message: message.to_string(),
            sha: Some(sha.to_string()),
        };
        self.github
            .delete_message(&self.path(location, ""), json!(file))
    }
}

/// Contents of a path in a repository.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Contents {
    File(File),
    Symlink(Symlink),
    Submodule(Submodule),
}

/// The type of content encoding.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Encoding {
    Base64,
    // Are there actually any other encoding types?
}

#[derive(Debug, Serialize)]
pub struct NewFile {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub content: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewFileResponse {
    pub commit: CommitDetails,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub encoding: Encoding,
    pub size: u32,
    pub name: String,
    pub path: String,
    pub content: DecodedContents,
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

/// Decoded file contents.
#[derive(Debug)]
pub struct DecodedContents(Vec<u8>);

impl From<DecodedContents> for Vec<u8> {
    fn from(DecodedContents(bytes): DecodedContents) -> Vec<u8> {
        bytes
    }
}

impl AsRef<[u8]> for DecodedContents {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ops::Deref for DecodedContents {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for DecodedContents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DecodedContentsVisitor;

        impl<'de> Visitor<'de> for DecodedContentsVisitor {
            type Value = DecodedContents;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "base64 string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use base64::engine::{Engine, general_purpose::STANDARD};
                // GitHub wraps the base64 to column 60. The base64 crate
                // doesn't handle whitespace, nor does it take a reader, so we
                // must unfortunately allocate again and remove all new lines.
                let v = v.replace("\n", "");

                let decoded = STANDARD.decode(&v).map_err(|e| match e {
                    base64::DecodeError::InvalidLength => {
                        E::invalid_length(v.len(), &"invalid base64 length")
                    }
                    base64::DecodeError::InvalidPadding => {
                        E::invalid_length(v.len(), &"invalid base64 padding")
                    }
                    base64::DecodeError::InvalidByte(offset, byte) => E::invalid_value(
                        de::Unexpected::Bytes(&[byte]),
                        &format!("valid base64 character at offset {}", offset).as_str(),
                    ),
                    base64::DecodeError::InvalidLastSymbol(offset, byte) => E::invalid_value(
                        de::Unexpected::Bytes(&[byte]),
                        &format!("valid last base64 character at offset {}", offset).as_str(),
                    ),
                })?;

                Ok(DecodedContents(decoded))
            }
        }

        deserializer.deserialize_str(DecodedContentsVisitor)
    }
}
