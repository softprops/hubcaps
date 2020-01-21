//! Teams interface
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::users::User;
use crate::{Github, Result, Stream};

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

/// reference to teams associated with a github repo
pub struct RepoTeams {
    github: Github,
    owner: String,
    repo: String,
}

impl RepoTeams {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
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
    pub async fn list(&self) -> Result<Vec<Team>> {
        self.github
            .get(&format!("/repos/{}/{}/teams", self.owner, self.repo))
            .await
    }

    /// provides a stream over all pages of teams
    pub async fn iter(&self) -> Stream<Team> {
        self.github
            .get_stream(&format!("/repos/{}/{}/teams", self.owner, self.repo))
    }
}

/// reference to teams associated with a github org
pub struct OrgTeams {
    github: Github,
    org: String,
}

impl OrgTeams {
    #[doc(hidden)]
    pub fn new<O>(github: Github, org: O) -> Self
    where
        O: Into<String>,
    {
        OrgTeams {
            github,
            org: org.into(),
        }
    }

    /// list of teams for this org
    pub async fn list(&self) -> Result<Vec<Team>> {
        self.github.get(&format!("/orgs/{}/teams", self.org)).await
    }

    /// Get a reference to a structure for interfacing with a specific
    /// team
    pub fn get(&self, number: u64) -> OrgTeamActions {
        OrgTeamActions::new(self.github.clone(), number)
    }

    /// create team
    pub async fn create(&self, team_options: &TeamOptions) -> Result<Team> {
        self.github
            .post(&format!("/orgs/{}/teams", self.org), json!(team_options)?)
            .await
    }

    /// provides an iterator over all pages of teams
    pub async fn iter(&self) -> Stream<Team> {
        self.github.get_stream(&format!("/orgs/{}/teams", self.org))
    }

    /// adds a repository permission to this team
    /// learn more [here](https://developer.github.com/v3/orgs/teams/#add-or-update-team-repository)
    pub async fn add_repo_permission<N>(
        &self,
        team_id: u64,
        repo_name: N,
        permission: Permission,
    ) -> Result<()>
    where
        N: Into<String>,
    {
        self.github
            .put_no_response(
                &format!("/teams/{}/repos/{}/{}", team_id, self.org, repo_name.into()),
                json_lit!({ "permission": permission.to_string() })?,
            )
            .await
    }
}

/// reference to teams associated with a github org
pub struct OrgTeamActions {
    github: Github,
    number: u64,
}

impl OrgTeamActions {
    #[doc(hidden)]
    pub fn new(github: Github, number: u64) -> Self {
        OrgTeamActions { github, number }
    }

    fn path(&self, suffix: &str) -> String {
        format!("/teams/{}{}", self.number, suffix)
    }

    /// list the team
    pub async fn get(&self) -> Result<Team> {
        self.github.get(&self.path("")).await
    }

    /// edit the team
    pub async fn update(&self, team_options: &TeamOptions) -> Result<Team> {
        self.github
            .patch(&self.path(""), json!(team_options)?)
            .await
    }

    /// delete the team
    pub async fn delete(&self) -> Result<()> {
        self.github.delete(&self.path("")).await
    }

    /// list of teams for this org
    pub async fn list_members(&self) -> Result<Vec<User>> {
        self.github.get(&self.path("/members")).await
    }

    /// provides an iterator over all pages of members
    pub async fn iter_members(&self) -> Stream<User> {
        self.github.get_stream(&self.path("/members"))
    }

    /// add a user to the team, if they are already on the team,
    /// change the role. If the user is not yet part of the
    /// organization, they are invited to join.
    pub async fn add_user(
        &self,
        user: &str,
        user_options: TeamMemberOptions,
    ) -> Result<TeamMember> {
        self.github
            .put(
                &self.path(&format!("/memberships/{}", user)),
                json!(user_options)?,
            )
            .await
    }

    /// Remove the user from the team
    pub async fn remove_user(&self, user: &str) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/memberships/{}", user)))
            .await
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
