//! Organizations interface

use self::super::{Github, Result};
use rep::Org;

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
