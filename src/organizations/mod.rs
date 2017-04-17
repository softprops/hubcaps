//! Organizations interface

use self::super::{Github, Result};
use teams::OrgTeams;
use repositories::OrgRepositories;

/// Provides access to label operations available for an individual organization
pub struct Organization<'a> {
    github: &'a Github,
    org: String,
}

impl<'a> Organization<'a> {
    pub fn new<O>(github: &'a Github, org: O) -> Self
        where O: Into<String>
    {
        Organization {
            github: github,
            org: org.into(),
        }
    }

    /// returns a reference to an interface for team operations
    pub fn teams(&self) -> OrgTeams {
        OrgTeams::new(self.github, self.org.clone())
    }

    /// returns a reference to an interface for repo operations
    pub fn repos(&self) -> OrgRepositories {
        OrgRepositories::new(self.github, self.org.clone())
    }
}


pub struct Organizations<'a> {
    github: &'a Github,
}

impl<'a> Organizations<'a> {
    pub fn new(github: &'a Github) -> Organizations<'a> {
        Organizations { github: github }
    }

    fn path(&self, more: &str) -> String {
        format!("/user/orgs{}", more)
    }

    /// list the authenticated user's organizations
    /// https://developer.github.com/v3/orgs/#list-your-organizations
    pub fn list(&self) -> Result<Vec<Org>> {
        self.github.get::<Vec<Org>>(&self.path(""))
    }
}

pub struct UserOrganizations<'a> {
    github: &'a Github,
    user: String,
}

impl<'a> UserOrganizations<'a> {
    pub fn new<U>(github: &'a Github, user: U) -> UserOrganizations<'a>
        where U: Into<String>
    {
        UserOrganizations {
            github: github,
            user: user.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/users/{}/orgs{}", self.user, more)
    }

    /// list the organizations this user is publicly associated with
    /// https://developer.github.com/v3/orgs/#list-user-organizations
    pub fn list(&self) -> Result<Vec<Org>> {
        self.github.get::<Vec<Org>>(&self.path(""))
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
