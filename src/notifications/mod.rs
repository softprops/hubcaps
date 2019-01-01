//! Notifications interface
extern crate serde_json;

use std::collections::HashMap;

use hyper::client::connect::Connect;
use url::form_urlencoded;

use users::User;
use Future;
use Github;

/// Provides access to notifications.
/// See the [github docs](https://developer.github.com/v3/activity/notifications/)
/// for more information.
pub struct Notifications<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

impl<C: Clone + Connect + 'static> Notifications<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }

    /// List the authenticated user's notifications.
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#list-your-notifications)
    /// for more information.
    pub fn list(&self, options: &ThreadListOptions) -> Future<Vec<Thread>> {
        let mut uri = vec!["/notifications".into()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// List the authenticated user's notifications for a repository.
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#list-your-notifications-in-a-repository)
    /// for more information.
    pub fn list_for_repo<O, R>(
        &self,
        owner: O,
        repo: R,
        options: &ThreadListOptions,
    ) -> Future<Vec<Thread>>
    where
        O: Into<String>,
        R: Into<String>,
    {
        let mut uri = vec![format!(
            "/repos/{}/{}/notifications",
            owner.into(),
            repo.into()
        )];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// Mark notifications as read. Default: `now`
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#mark-as-read)
    /// for more information.
    pub fn mark_as_read<S>(&self, last_read_at: S) -> Future<()>
    where
        S: Into<Option<String>>,
    {
        let url = match last_read_at.into() {
            Some(last_read_at) => format!(
                "/notifications?{}",
                form_urlencoded::Serializer::new(String::new())
                    .append_pair("last_read_at", &last_read_at)
                    .finish()
            ),
            None => String::from("/notifications"),
        };
        self.github.put_no_response(&url, Vec::new())
    }

    /// Mark notifications as read in a repository. Default: `now`
    ///
    /// See [github docs](https://developer.github.com/v3/activity/notifications/#mark-notifications-as-read-in-a-repository)
    /// for more information.
    pub fn mark_as_read_for_repo<O, R, S>(&self, owner: O, repo: R, last_read_at: S) -> Future<()>
    where
        O: Into<String>,
        R: Into<String>,
        S: Into<Option<String>>,
    {
        let path = match last_read_at.into() {
            Some(last_read_at) => format!(
                "/notifications?{}",
                form_urlencoded::Serializer::new(String::new())
                    .append_pair("last_read_at", &last_read_at)
                    .finish()
            ),
            None => String::from("/notifications"),
        };
        self.github.put_no_response(
            &format!("/repos/{}/{}{}", owner.into(), repo.into(), path),
            Vec::new(),
        )
    }

    /// Return a single thread.
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#view-a-single-thread)
    /// for more information.
    pub fn get_thread<S>(&self, id: S) -> Future<Thread>
    where
        S: Into<String>,
    {
        self.github
            .get(&format!("/notifications/threads/{}", id.into()))
    }

    /// Mark a thread as read
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#mark-a-thread-as-read)
    /// for more information.
    pub fn mark_thread_as_read<S>(&self, id: S) -> Future<()>
    where
        S: Into<String>,
    {
        self.github
            .patch_no_response(&format!("/notifications/threads/{}", id.into()), Vec::new())
    }

    /// Return the subscription information for a thread.
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#get-a-thread-subscription)
    /// for more information.
    pub fn get_subscription<S>(&self, id: S) -> Future<Subscription>
    where
        S: Into<String>,
    {
        self.github.get(&format!(
            "/notifications/threads/{}/subscription",
            id.into(),
        ))
    }

    /// Subscribe to a thread and return the subscription information.
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#set-a-thread-subscription)
    /// for more information.
    pub fn subscribe<S>(&self, id: S) -> Future<Subscription>
    where
        S: Into<String>,
    {
        self.github.put(
            &format!("/notifications/threads/{}/subscription", id.into()),
            json_lit!({ "subscribed": true }),
        )
    }

    /// Unsubscribe to a thread and return the subscription information.
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#set-a-thread-subscription)
    /// for more information.
    pub fn unsubscribe<S>(&self, id: S) -> Future<Subscription>
    where
        S: Into<String>,
    {
        self.github.put(
            &format!("/notifications/threads/{}/subscription", id.into()),
            json_lit!({ "ignored": true }),
        )
    }

    /// Delete the thread subscription.
    ///
    /// See the [github docs](https://developer.github.com/v3/activity/notifications/#delete-a-thread-subscription)
    /// for more information.
    pub fn delete_subscription<S>(&self, id: S) -> Future<()>
    where
        S: Into<String>,
    {
        self.github.delete(&format!(
            "/notifications/threads/{}/subscription",
            id.into()
        ))
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Thread {
    pub id: String,
    pub unread: bool,
    pub updated_at: String,
    pub last_read_at: Option<String>,
    pub reason: String,
    pub subject: Subject,
    pub repository: Repository,
    pub url: String,
    pub subscription_url: String,
}

#[derive(Default)]
pub struct ThreadListOptions {
    params: HashMap<&'static str, String>,
}

impl ThreadListOptions {
    pub fn builder() -> ThreadListOptionsBuilder {
        ThreadListOptionsBuilder::default()
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
pub struct ThreadListOptionsBuilder(ThreadListOptions);

impl ThreadListOptionsBuilder {
    /// if `true`, show notifications marked as read. Default: `false`
    pub fn all(&mut self, all: bool) -> &mut Self {
        self.0.params.insert("all", all.to_string());
        self
    }

    /// if `true`, only shows notifications in which the user is directly participating or
    /// mentioned. Default: `false`
    pub fn participating(&mut self, val: bool) -> &mut Self {
        self.0.params.insert("participating", val.to_string());
        self
    }

    /// Only show notifications updated after the given time.
    pub fn since<T>(&mut self, since: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.0.params.insert("since", since.into());
        self
    }

    /// Only show notifications updated before a given time.
    pub fn before<T>(&mut self, before: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.0.params.insert("before", before.into());
        self
    }

    pub fn build(&self) -> ThreadListOptions {
        ThreadListOptions {
            params: self.0.params.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Subject {
    title: String,
    url: String,
    latest_comment_url: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Subscription {
    pub subscribed: bool,
    pub ignored: bool,
    pub reason: String,
    pub created_at: String,
    pub url: String,
    pub thread_url: String,
}
