//! Review requests interface
use hyper::client::connect::Connect;
use serde::{Deserialize, Serialize};

use crate::pulls::Pull;
use crate::teams::Team;
use crate::users::User;
use crate::{Error, Future, Github};

/// A structure for interfacing with review requests
pub struct ReviewRequests<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
    number: u64,
}

impl<C: Clone + Connect + 'static> ReviewRequests<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, number: u64) -> Self
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
    pub fn get(&self) -> impl Future<Item = ReviewRequest, Error = Error> {
        self.github.get::<ReviewRequest>(&self.path())
    }

    /// Add new requested reviews
    pub fn create(&self, review_request: &ReviewRequestOptions) -> impl Future<Item = Pull, Error = Error> {
        self.github.post(&self.path(), json!(review_request))
    }

    /// Delete a review request
    pub fn delete(&self, review_request: &ReviewRequestOptions) -> impl Future<Item = (), Error = Error> {
        self.github
            .delete_message(&self.path(), json!(review_request))
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
