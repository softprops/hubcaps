//! Organizations interface

use hyper::client::connect::Connect;

use {Future, Github};
use repositories::OrgRepositories;
use teams::OrgTeams;

/// Provides access to label operations available for an individual organization
pub struct Organization<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    org: String,
}

impl<C: Clone + Connect + 'static> Organization<C> {
    #[doc(hidden)]
    pub fn new<O>(github: Github<C>, org: O) -> Self
    where
        O: Into<String>,
    {
        Organization {
            github,
            org: org.into(),
        }
    }

    /// returns a reference to an interface for team operations
    pub fn teams(&self) -> OrgTeams<C> {
        OrgTeams::new(self.github.clone(), self.org.clone())
    }

    /// returns a reference to an interface for repo operations
    pub fn repos(&self) -> OrgRepositories<C> {
        OrgRepositories::new(self.github.clone(), self.org.clone())
    }
}

pub struct Organizations<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

impl<C: Clone + Connect + 'static> Organizations<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }

    fn path(&self, more: &str) -> String {
        format!("/user/orgs{}", more)
    }

    /// list the authenticated user's organizations
    /// https://developer.github.com/v3/orgs/#list-your-organizations
    pub fn list(&self) -> Future<Vec<Org>> {
        self.github.get(&self.path(""))
    }
}

pub struct UserOrganizations<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    user: String,
}

impl<C: Clone + Connect + 'static> UserOrganizations<C> {
    pub fn new<U>(github: Github<C>, user: U) -> Self
    where
        U: Into<String>,
    {
        UserOrganizations {
            github,
            user: user.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/users/{}/orgs{}", self.user, more)
    }

    /// list the organizations this user is publicly associated with
    /// https://developer.github.com/v3/orgs/#list-user-organizations
    pub fn list(&self) -> Future<Vec<Org>> {
        self.github.get(&self.path(""))
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Org {
    pub login: String,
    pub id: u64,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: Option<String>,
}
