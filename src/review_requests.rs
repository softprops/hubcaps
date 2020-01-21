//! Review requests interface
use serde::{Deserialize, Serialize};

use crate::pulls::Pull;
use crate::teams::Team;
use crate::users::User;
use crate::{Github, Result};

/// A structure for interfacing with review requests
pub struct ReviewRequests {
    github: Github,
    owner: String,
    repo: String,
    number: u64,
}

impl ReviewRequests {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        ReviewRequests {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    /// list requested reviews
    pub async fn get(&self) -> Result<ReviewRequest> {
        self.github.get::<ReviewRequest>(&self.path()).await
    }

    /// Add new requested reviews
    pub async fn create(&self, review_request: &ReviewRequestOptions) -> Result<Pull> {
        self.github.post(&self.path(), json!(review_request)?).await
    }

    /// Delete a review request
    pub async fn delete(&self, review_request: &ReviewRequestOptions) -> Result<()> {
        self.github
            .delete_message(&self.path(), json!(review_request)?)
            .await
    }

    fn path(&self) -> String {
        format!(
            "/repos/{}/{}/pulls/{}/requested_reviewers",
            self.owner, self.repo, self.number
        )
    }
}

// representations (todo: replace with derive_builder)

#[derive(Default, Serialize)]
pub struct ReviewRequestOptions {
    /// An array of user `logins` that will be requested.
    /// Note, each login must be a collaborator.
    pub reviewers: Vec<String>,
    /// An array of team `slugs` that will be requested.
    pub team_reviewers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewRequest {
    pub users: Vec<User>,
    pub teams: Vec<Team>,
}
