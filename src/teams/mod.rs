//! Teams interface
use self::super::{Iter, Github, Result};
use std::fmt;
use serde_json;
use std::collections::BTreeMap;

/// Team repository permissions
pub enum Permission {
    Pull,
    Push,
    Admin,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Permission::Pull => "pull",
            Permission::Push => "push",
            Permission::Admin => "admin",
        }.fmt(f)
    }
}

fn identity<T>(x: T) -> T {
    x
}

/// reference to teams associated with a github repo
pub struct RepoTeams<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> RepoTeams<'a> {
    #[doc(hidden)]
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        RepoTeams {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list of teams for this repo
    pub fn list(&self) -> Result<Vec<Team>> {
        self.github.get(&format!(
            "/repos/{}/{}/teams",
            self.owner,
            self.repo
        ))
    }

    /// provides an iterator over all pages of teams
    pub fn iter(&'a self) -> Result<Iter<'a, Vec<Team>, Team>> {
        self.github.iter(
            format!("/repos/{}/{}/teams", self.owner, self.repo),
            identity,
        )
    }
}

/// reference to teams associated with a github org
pub struct OrgTeams<'a> {
    github: &'a Github,
    org: String,
}

impl<'a> OrgTeams<'a> {
    #[doc(hidden)]
    pub fn new<O>(github: &'a Github, org: O) -> Self
    where
        O: Into<String>,
    {
        OrgTeams {
            github: github,
            org: org.into(),
        }
    }

    /// list of teams for this org
    pub fn list(&self) -> Result<Vec<Team>> {
        self.github.get(&format!("/orgs/{}/teams", self.org))
    }

    /// provides an iterator over all pages of teams
    pub fn iter(&'a self) -> Result<Iter<'a, Vec<Team>, Team>> {
        self.github.iter(
            format!("/orgs/{}/teams", self.org),
            identity,
        )
    }

    /// adds a repository permission to this team
    /// learn more [here](https://developer.github.com/v3/orgs/teams/#add-or-update-team-repository)
    pub fn add_repo_permission<N>(
        &self,
        team_id: u64,
        repo_name: N,
        permission: Permission,
    ) -> Result<()>
    where
        N: Into<String>,
    {
        let mut payload = BTreeMap::new();
        payload.insert("permission", permission.to_string());
        let data = serde_json::to_string(&payload)?;
        self.github.put_no_response(
            &format!("/teams/{}/repos/{}/{}", team_id, self.org, repo_name.into()),
            data.as_bytes(),
        )
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: String,
    pub members_url: String,
    pub repositories_url: String,
    pub permission: String,
}
