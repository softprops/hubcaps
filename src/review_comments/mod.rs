//! Review comments interface

extern crate futures;
extern crate serde_json;

use hyper::client::connect::Connect;

use users::User;
use {Future, Github};

/// A structure for interfacing with a review comments
pub struct ReviewComments<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
    number: u64,
}

impl<C: Clone + Connect + 'static> ReviewComments<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        ReviewComments {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    /// list review comments
    pub fn list(&self) -> Future<Vec<ReviewComment>> {
        self.github.get::<Vec<ReviewComment>>(&self.path())
    }

    /// Create new review comment
    pub fn create(&self, review_comment: &ReviewCommentOptions) -> Future<ReviewComment> {
        self.github.post(&self.path(), json!(review_comment))
    }

    fn path(&self) -> String {
        format!(
            "/repos/{}/{}/pulls/{}/comments",
            self.owner, self.repo, self.number
        )
    }
}

// representations (todo: replace with derive_builder)

#[derive(Default, Serialize)]
pub struct ReviewCommentOptions {
    pub body: String,
    pub commit_id: String,
    pub path: String,
    pub position: usize,
}

#[derive(Debug, Deserialize)]
pub struct ReviewComment {
    pub id: u64,
    pub url: String,
    pub diff_hunk: String,
    pub path: String,
    pub position: u64,
    pub original_position: u64,
    pub commit_id: String,
    pub original_commit_id: String,
    pub user: User,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
    pub pull_request_url: String,
}
