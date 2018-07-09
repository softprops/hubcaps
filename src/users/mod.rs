//! Users interface

use {Future, Github};

use hyper::client::connect::Connect;

/// User information
#[derive(Debug, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    // type (keyword)
    pub site_admin: bool,
}

/// Information about current authenticated user
#[derive(Debug, Deserialize)]
pub struct AuthenticatedUser {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    // type (keyword)
    pub site_admin: bool,

    // extend over `User`:
    pub name: String,
    pub company: Option<String>,
    pub blog: String,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub public_repos: u64,
    pub public_gists: u64,
    pub followers: u64,
    pub following: u64,
    pub created_at: String, // TODO: change to `DateTime`?
    pub updated_at: String, // TODO: change to `DateTime`?
}

/// Query user information
pub struct Users<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

impl<C: Clone + Connect + 'static> Users<C> {
    pub fn new(github: Github<C>) -> Self {
        Users { github }
    }

    /// Information about current authenticated user
    pub fn authenticated(&self) -> Future<AuthenticatedUser> {
        self.github.get("/user")
    }

    pub fn get<U>(&self, username: U) -> Future<User>
    where
        U: Into<String>,
    {
        self.github
            .get(&format!("/users/{username}", username = username.into()))
    }
}
