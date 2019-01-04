//! Teams interface
use std::fmt;

use hyper::client::connect::Connect;
use serde::{Deserialize, Serialize};

use crate::users::User;
use crate::{unfold, Future, Github, Stream};

/// Team repository permissions
#[derive(Clone, Copy)]
pub enum Permission {
    Pull,
    Push,
    Admin,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    /// Get a reference to a structure for interfacing with a specific
    /// team
    pub fn get(&self, number: u64) -> OrgTeamActions<C> {
        OrgTeamActions::new(self.github.clone(), number)
    }

    /// create team
    pub fn create(&self, team_options: &TeamOptions) -> Future<Team> {
        self.github
            .post(&format!("/orgs/{}/teams", self.org), json!(team_options))
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

/// reference to teams associated with a github org
pub struct OrgTeamActions<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    number: u64,
}

impl<C: Clone + Connect + 'static> OrgTeamActions<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>, number: u64) -> Self {
        OrgTeamActions { github, number }
    }

    fn path(&self, suffix: &str) -> String {
        format!("/teams/{}{}", self.number, suffix)
    }

    /// list the team
    pub fn get(&self) -> Future<Team> {
        self.github.get(&self.path(""))
    }

    /// edit the team
    pub fn update(&self, team_options: &TeamOptions) -> Future<Team> {
        self.github.patch(&self.path(""), json!(team_options))
    }

    /// delete the team
    pub fn delete(&self) -> Future<()> {
        self.github.delete(&self.path(""))
    }

    /// list of teams for this org
    pub fn list_members(&self) -> Future<Vec<User>> {
        self.github.get(&self.path("/members"))
    }

    /// provides an iterator over all pages of members
    pub fn iter_members(&self) -> Stream<User> {
        unfold(
            self.github.clone(),
            self.github.get_pages(&self.path("/members")),
            identity,
        )
    }

    /// add a user to the team, if they are already on the team,
    /// change the role. If the user is not yet part of the
    /// organization, they are invited to join.
    pub fn add_user(&self, user: &str, user_options: TeamMemberOptions) -> Future<TeamMember> {
        self.github.put(
            &self.path(&format!("/memberships/{}", user)),
            json!(user_options),
        )
    }

    /// Remove the user from the team
    pub fn remove_user(&self, user: &str) -> Future<()> {
        self.github
            .delete(&self.path(&format!("/memberships/{}", user)))
    }
}

// representations (todo: replace with derive_builder)

#[derive(Debug, Deserialize)]
pub struct TeamMember {
    pub url: String,
    pub role: TeamMemberRole,
    pub state: TeamMemberState,
}

#[derive(Debug, Serialize)]
pub struct TeamMemberOptions {
    pub role: TeamMemberRole,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TeamMemberRole {
    Member,
    Maintainer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TeamMemberState {
    Active,
    Pending,
}

/// Representation of a specific team
#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: String,
    pub permission: String,
    pub members_url: String,
    pub repositories_url: String,
}

#[derive(Debug, Serialize)]
pub struct TeamOptions {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}
