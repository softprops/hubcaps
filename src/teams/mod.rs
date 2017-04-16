//! Gists interface
extern crate serde_json;

use self::super::{Error, Github, Result};
use url::form_urlencoded;
use users::User;
use std::collections::HashMap;
use std::hash::Hash;

pub struct RepoTeams<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> RepoTeams<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Self
        where O: Into<String>,
              R: Into<String>
    {
        RepoTeams {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list of teams for this repo
    pub fn list(&self) -> Result<Vec<Team>> {
        self.github
            .get::<Vec<Team>>(&format!("/repos/{}/{}/teams", self.owner, self.repo))
    }
}

/// reference to gists associated with a github user
pub struct OrgTeams<'a> {
    github: &'a Github,
    org: String,
}

impl<'a> OrgTeams<'a> {
    pub fn new<O>(github: &'a Github, org: O) -> Self
        where O: Into<String>
    {
        OrgTeams {
            github: github,
            org: org.into(),
        }
    }

    /// list of teams for this org
    pub fn list(&self) -> Result<Vec<Team>> {
        self.github
            .get::<Vec<Team>>(&format!("/orgs/{}/teams", self.org))
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub privacy: String,
    pub members_url: String,
    pub repositories_url: String,
    pub permission: String,
}
