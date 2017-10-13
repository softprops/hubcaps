//! Repo branches interface
//!
//! For more information, visit the official
//! [Github docs](https://developer.github.com/v3/repos/branches/)
extern crate serde_json;

use self::super::{Iter, Github, Result};

fn identity<T>(x: T) -> T {
    x
}

/// reference to gists associated with a github user
pub struct Branches<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Branches<'a> {
    #[doc(hidden)]
    pub fn new<U, R>(github: &'a Github, owner: U, repo: R) -> Self
    where
        U: Into<String>,
        R: Into<String>,
    {
        Branches {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list of branches for this repo
    pub fn list(&self) -> Result<Vec<Branch>> {
        self.github.get(&format!(
            "/repos/{owner}/{repo}/branches",
            owner = self.owner,
            repo = self.repo
        ))
    }

    /// provides an iterator over branches for this repo
    pub fn iter(&self) -> Result<Iter<Vec<Branch>, Branch>> {
        self.github.iter(
            format!(
                "/repos/{owner}/{repo}/branches",
                owner = self.owner,
                repo = self.repo
            ),
            identity,
        )
    }

    /// gets a branch for this repo by name
    pub fn get<B>(&self, branch: B) -> Result<Branch>
    where
        B: Into<String>,
    {
        self.github.get(&format!(
            "/repos/{owner}/{repo}/branches/{branch}",
            owner = self.owner,
            repo = self.repo,
            branch = branch.into()
        ))
    }

    /// update branch production for a given branch
    ///
    /// https://developer.github.com/v3/repos/branches/#update-branch-protection
    pub fn protection<B>(&self, branch: B, pro: &Protection) -> Result<Protection>
    where
        B: Into<String>,
    {
        self.github.put(
            &format!(
                "/repos/{owner}/{repo}/branches/{branch}/protection",
                owner = self.owner,
                repo = self.repo,
                branch = branch.into()
            ),
            &serde_json::to_vec(&pro)?,
        )
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
    pub protected: Option<bool>,
    pub protection_url: Option<String>,
    // todo: commit ref
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Protection {
    pub required_status_checks: Option<StatusChecks>,
    pub enforce_admins: bool,
    pub required_pull_request_reviews: Option<RequiredPullRequestReviews>,
    pub restrictions: Option<Restrictions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Restrictions {
    pub users: Vec<String>,
    pub teams: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequiredPullRequestReviews {
    pub dismissal_restrictions: Restrictions,
    pub dismiss_stale_reviews: bool,
    pub require_code_owner_reviews: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusChecks {
    pub strict: bool,
    pub contexts: Vec<String>,
}
