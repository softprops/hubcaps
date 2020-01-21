//! Watching interface
/// https://developer.github.com/v3/activity/watching
use serde::Deserialize;

use crate::repositories::Repo;
use crate::{Github, Result, Stream};

pub struct Watching {
    github: Github,
}

impl Watching {
    #[doc(hidden)]
    pub fn new(github: Github) -> Self {
        Self { github }
    }

    /// Provides a stream over all pages of the repositories watched by the authenticated user.
    /// https://developer.github.com/v3/activity/watching/#list-repositories-being-watched
    pub async fn iter(&self) -> Stream<Repo> {
        self.github.get_stream("/user/subscriptions")
    }

    /// https://developer.github.com/v3/activity/watching/#get-a-repository-subscription
    pub async fn get_for_repo<O, R>(&self, owner: O, repo: R) -> Result<Subscription>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .get(&format!(
                "/repos/{}/{}/subscription",
                owner.into(),
                repo.into()
            ))
            .await
    }

    /// https://developer.github.com/v3/activity/watching/#set-a-repository-subscription
    pub async fn watch_repo<O, R>(&self, owner: O, repo: R) -> Result<Subscription>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .put(
                &format!("/repos/{}/{}/subscription", owner.into(), repo.into()),
                json_lit!({ "subscribed": true })?,
            )
            .await
    }

    /// https://developer.github.com/v3/activity/watching/#set-a-repository-subscription
    pub async fn ignore_repo<O, R>(&self, owner: O, repo: R) -> Result<Subscription>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .put(
                &format!("/repos/{}/{}/subscription", owner.into(), repo.into()),
                json_lit!({ "ignored": true })?,
            )
            .await
    }

    /// https://developer.github.com/v3/activity/watching/#set-a-repository-subscription
    pub async fn unwatch_repo<O, R>(&self, owner: O, repo: R) -> Result<()>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .delete(&format!(
                "/repos/{}/{}/subscription",
                owner.into(),
                repo.into()
            ))
            .await
    }
}

#[derive(Debug, Deserialize)]
pub struct Subscription {
    pub subscribed: bool,
    pub ignored: bool,
    pub reason: Option<String>,
    pub created_at: String,
    pub url: String,
    pub repository_url: String,
}
