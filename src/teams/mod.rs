//! Teams interface
use self::super::{Iter, Github, Result};

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

    /// provides an iterator over all pages of teams
    pub fn iter(&'a self) -> Result<Iter<'a, Vec<Team>, Team>> {
        self.github
            .iter(format!("/repos/{}/{}/teams", self.owner, self.repo),
                  identity)
    }
}

/// reference to teams associated with a github org
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

    /// provides an iterator over all pages of teams
    pub fn iter(&'a self) -> Result<Iter<'a, Vec<Team>, Team>> {
        self.github
            .iter(format!("/orgs/{}/teams", self.org), identity)
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
