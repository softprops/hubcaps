//! Repo branches interface
//!
//! For more information, visit the official
//! [Github docs](https://developer.github.com/v3/repos/branches/)
use serde::{Deserialize, Serialize};

use crate::{Future, Github, MediaType, Stream};

/// reference to gists associated with a github user
pub struct Branches {
    github: Github,
    owner: String,
    repo: String,
}

impl Branches {
    #[doc(hidden)]
    pub fn new<U, R>(github: Github, owner: U, repo: R) -> Self
    where
        U: Into<String>,
        R: Into<String>,
    {
        Branches {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list of branches for this repo
    pub fn list(&self) -> Future<Vec<Branch>> {
        self.github.get(&format!(
            "/repos/{owner}/{repo}/branches",
            owner = self.owner,
            repo = self.repo
        ))
    }

    /// provides an stream over branches for this repo
    pub fn iter(&self) -> Stream<Branch> {
        self.github.get_stream(&format!(
            "/repos/{owner}/{repo}/branches",
            owner = self.owner,
            repo = self.repo
        ))
    }

    /// gets a branch for this repo by name
    pub fn get<B>(&self, branch: B) -> Future<Branch>
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
    pub fn protection<B>(&self, branch: B, pro: &Protection) -> Future<ProtectionState>
    where
        B: Into<String>,
    {
        self.github.put_media(
            &format!(
                "/repos/{owner}/{repo}/branches/{branch}/protection",
                owner = self.owner,
                repo = self.repo,
                branch = branch.into()
            ),
            json!(pro),
            MediaType::Preview("luke-cage"),
        )
    }

    /// Rename a branch
    ///
    /// https://docs.github.com/en/rest/reference/repos#rename-a-branch
    pub fn rename<B>(&self, branch: B, rename: &Rename) -> Future<Branch>
    where
        B: Into<String>,
    {
        self.github.post(
            &format!(
                "/repos/{owner}/{repo}/branches/{branch}/rename",
                owner = self.owner,
                repo = self.repo,
                branch = branch.into()
            ),
            json!(rename),
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
pub struct ProtectionState {
    pub required_status_checks: Option<StatusChecks>,
    pub enforce_admins: Option<EnforceAdmins>,
    //pub required_pull_request_reviews: Option<RequiredPullRequestReviews>,
    //pub restrictions: Option<Restrictions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnforceAdmins {
    pub url: String,
    pub enabled: bool,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions: Option<Restrictions>,
    pub dismiss_stale_reviews: bool,
    pub require_code_owner_reviews: bool,
    pub required_approving_review_count: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusChecks {
    pub strict: bool,
    pub contexts: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rename {
    pub new_name: String,
}
