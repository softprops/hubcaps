//! Labels interface
use serde::Deserialize;

use self::super::{AuthenticationConstraint, Github, MediaType, Result};

pub struct App {
    github: Github,
}

impl App {
    #[doc(hidden)]
    pub(crate) fn new(github: Github) -> Self {
        App { github: github }
    }

    fn path(&self, more: &str) -> String {
        format!("/app{}", more)
    }

    pub async fn make_access_token(&self, installation_id: u64) -> Result<AccessToken> {
        self.github
            .post_media::<AccessToken>(
                &self.path(&format!("/installations/{}/access_tokens", installation_id)),
                Vec::new(),
                MediaType::Preview("machine-man"),
                AuthenticationConstraint::JWT,
            )
            .await
    }

    pub async fn find_repo_installation<O, R>(&self, owner: O, repo: R) -> Result<Installation>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github
            .get_media::<Installation>(
                &format!("/repos/{}/{}/installation", owner.into(), repo.into()),
                MediaType::Preview("machine-man"),
            )
            .await
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
pub struct Installation {
    pub id: u64,
    // account: Account
    pub access_tokens_url: String,
    pub repositories_url: String,
    pub html_url: String,
    pub app_id: i32,
    pub target_id: i32,
    pub target_type: String,
    // permissions: Permissions
    pub events: Vec<String>,
    // created_at, updated_at
    pub single_file_name: Option<String>,
    pub repository_selection: String,
}
