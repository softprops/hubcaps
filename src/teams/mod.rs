//! Teams interface

use std::fmt;

use hyper::client::connect::Connect;
use serde_json;

use {unfold, Future, Github, Stream};

/// Team repository permissions
#[derive(Clone, Copy)]
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
        }
        .fmt(f)
    }
}

fn identity<T>(x: T) -> T {
    x
}

/// reference to teams associated with a github repo
pub struct RepoTeams<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> RepoTeams<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        RepoTeams {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list of teams for this repo
    pub fn list(&self) -> Future<Vec<Team>> {
        self.github
            .get(&format!("/repos/{}/{}/teams", self.owner, self.repo))
    }

    /// provides a stream over all pages of teams
    pub fn iter(&self) -> Stream<Team> {
        unfold(
            self.github.clone(),
            self.github
                .get_pages(&format!("/repos/{}/{}/teams", self.owner, self.repo)),
            identity,
        )
    }
}

/// reference to teams associated with a github org
pub struct OrgTeams<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    org: String,
}

impl<C: Clone + Connect + 'static> OrgTeams<C> {
    #[doc(hidden)]
    pub fn new<O>(github: Github<C>, org: O) -> Self
    where
        O: Into<String>,
    {
        OrgTeams {
            github,
            org: org.into(),
        }
    }

    /// list of teams for this org
    pub fn list(&self) -> Future<Vec<Team>> {
        self.github.get(&format!("/orgs/{}/teams", self.org))
    }

    /// provides an iterator over all pages of teams
    pub fn iter(&self) -> Stream<Team> {
        unfold(
            self.github.clone(),
            self.github.get_pages(&format!("/orgs/{}/teams", self.org)),
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
    ) -> Future<()>
    where
        N: Into<String>,
    {
        self.github.put_no_response(
            &format!("/teams/{}/repos/{}/{}", team_id, self.org, repo_name.into()),
            json_lit!({ "permission": permission.to_string() }),
        )
    }
}

// representations (todo: replace with derive_builder)

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
