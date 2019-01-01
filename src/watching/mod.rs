//! Watching interface
/// https://developer.github.com/v3/activity/watching
use hyper::client::connect::Connect;

use repositories::Repo;
use {unfold, Future, Github, Stream};

pub struct Watching<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

impl<C: Clone + Connect + 'static> Watching<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }

    /// Provides a stream over all pages of the repositories watched by the authenticated user.
    /// https://developer.github.com/v3/activity/watching/#list-repositories-being-watched
    pub fn iter(&self) -> Stream<Repo> {
        unfold(
            self.github.clone(),
            self.github.get_pages("/user/subscriptions"),
            |x| x,
        )
    }

    /// https://developer.github.com/v3/activity/watching/#get-a-repository-subscription
    pub fn get_for_repo<O, R>(&self, owner: O, repo: R) -> Future<Subscription> where
        O: Into<String>,
        R: Into<String>,
    {
        self.github.get(&format!("/repos/{}/{}/subscription", owner.into(), repo.into()))
    }

    /// https://developer.github.com/v3/activity/watching/#set-a-repository-subscription
    pub fn watch_repo<O, R>(&self, owner: O, repo: R) -> Future<Subscription> where
        O: Into<String>,
        R: Into<String>,
    {
        self.github.put(
            &format!("/repos/{}/{}/subscription", owner.into(), repo.into()),
            json_lit!({ "subscribed": true }),
        )
    }

    /// https://developer.github.com/v3/activity/watching/#set-a-repository-subscription
    pub fn ignore_repo<O, R>(&self, owner: O, repo: R) -> Future<Subscription> where
        O: Into<String>,
        R: Into<String>,
    {
        self.github.put(
            &format!("/repos/{}/{}/subscription", owner.into(), repo.into()),
            json_lit!({ "ignored": true }),
        )
    }

    /// https://developer.github.com/v3/activity/watching/#set-a-repository-subscription
    pub fn unwatch_repo<O, R>(&self, owner: O, repo: R) -> Future<()> where
        O: Into<String>,
        R: Into<String>,
    {
        self.github.delete(&format!("/repos/{}/{}/subscription", owner.into(), repo.into()))
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
