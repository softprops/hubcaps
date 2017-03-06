//! Comments interface

use super::{Github, Result};
use std::collections::HashMap;
use url::form_urlencoded;
use rep::User;

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

// representations

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub url: String,
    pub html_url: String,
    pub body: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default)]
pub struct CommentListOptions {
    params: HashMap<&'static str, String>,
}

impl CommentListOptions {
    pub fn builder() -> CommentListOptionsBuilder {
        CommentListOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded)
        }
    }
}

#[derive(Default)]
pub struct CommentListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl CommentListOptionsBuilder {
    pub fn new() -> CommentListOptionsBuilder {
        CommentListOptionsBuilder { ..Default::default() }
    }

    pub fn since<S>(&mut self, since: S) -> &mut CommentListOptionsBuilder
        where S: Into<String>
    {
        self.params.insert("since", since.into());
        self
    }

    pub fn build(&self) -> CommentListOptions {
        CommentListOptions { params: self.params.clone() }
    }
}
