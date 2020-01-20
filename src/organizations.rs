//! Organizations interface
use serde::Deserialize;

use crate::membership::OrgMembership;
use crate::repositories::OrgRepositories;
use crate::teams::OrgTeams;
use crate::{Future, Github};

/// Provides access to label operations available for an individual organization
pub struct Organization {
    github: Github,
    org: String,
}

impl Organization {
    #[doc(hidden)]
    pub fn new<O>(github: Github, org: O) -> Self
    where
        O: Into<String>,
    {
        Organization {
            github,
            org: org.into(),
        }
    }

    /// returns a reference to an interface for Organization invitations
    pub fn membership(&self) -> OrgMembership {
        OrgMembership::new(self.github.clone(), self.org.clone())
    }

    /// returns a reference to an interface for team operations
    pub fn teams(&self) -> OrgTeams {
        OrgTeams::new(self.github.clone(), self.org.clone())
    }

    /// returns a reference to an interface for repo operations
    pub fn repos(&self) -> OrgRepositories {
        OrgRepositories::new(self.github.clone(), self.org.clone())
    }
}

pub struct Organizations {
    github: Github,
}

impl Organizations {
    #[doc(hidden)]
    pub fn new(github: Github) -> Self {
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

pub struct UserOrganizations {
    github: Github,
    user: String,
}

impl UserOrganizations {
    pub fn new<U>(github: Github, user: U) -> Self
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
