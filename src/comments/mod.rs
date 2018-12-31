//! Comments interface

extern crate futures;
extern crate serde_json;

use hyper::client::connect::Connect;
use std::collections::HashMap;
use url::form_urlencoded;
use users::User;
use {Future, Github};

/// A structure for interfacing with a issue comments
pub struct Comments<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
    number: u64,
}

impl<C: Clone + Connect + 'static> Comments<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Comments {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    /// add a new comment
    pub fn create(&self, comment: &CommentOptions) -> Future<Comment> {
        self.github.post(&self.path(), json!(comment))
    }

    /// list pull requests
    pub fn list(&self, options: &CommentListOptions) -> Future<Vec<Comment>> {
        let mut uri = vec![self.path()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    fn path(&self) -> String {
        format!(
            "/repos/{}/{}/issues/{}/comments",
            self.owner, self.repo, self.number
        )
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

#[derive(Debug, Serialize)]
pub struct CommentOptions {
    pub body: String,
}

#[derive(Default)]
pub struct CommentListOptions {
    params: HashMap<&'static str, String>,
}

impl CommentListOptions {
    pub fn builder() -> CommentListOptionsBuilder {
        CommentListOptionsBuilder::default()
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
pub struct CommentListOptionsBuilder(CommentListOptions);

impl CommentListOptionsBuilder {
    pub fn since<S>(&mut self, since: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.0.params.insert("since", since.into());
        self
    }

    pub fn build(&self) -> CommentListOptions {
        CommentListOptions {
            params: self.0.params.clone(),
        }
    }
}
