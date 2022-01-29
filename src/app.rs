use crate::jwt::JWTCredentials;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use self::super::{AuthenticationConstraint, Future, Github, MediaType};

pub struct App {
    github: Github,
}

impl App {
    #[doc(hidden)]
    pub(crate) fn new(github: Github) -> Self {
        App { github }
    }

    fn path(&self, more: &str) -> String {
        format!("/app{}", more)
    }

    pub fn make_access_token(&self, installation_id: u64) -> Future<AccessToken> {
        self.github.post_media::<AccessToken>(
            &self.path(&format!("/installations/{}/access_tokens", installation_id)),
            Vec::new(),
            MediaType::Preview("machine-man"),
            AuthenticationConstraint::JWT,
        )
    }

    pub fn find_repo_installation<O, R>(&self, owner: O, repo: R) -> Future<Installation>
    where
        O: Into<String>,
        R: Into<String>,
    {
        self.github.get_media::<Installation>(
            &format!("/repos/{}/{}/installation", owner.into(), repo.into()),
            MediaType::Preview("machine-man"),
        )
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

/// A caching token "generator" which contains JWT credentials.
///
/// The authentication mechanism in the GitHub client library
/// determines if the token is stale, and if so, uses the contained
/// JWT credentials to fetch a new installation token.
///
/// The Mutex<Option> access key is for interior mutability.
#[derive(Debug, Clone)]
pub struct InstallationTokenGenerator {
    pub installation_id: u64,
    pub jwt_credential: Box<crate::Credentials>,
    pub(crate) access_key: Arc<Mutex<Option<String>>>,
}

impl InstallationTokenGenerator {
    pub fn new(installation_id: u64, creds: JWTCredentials) -> InstallationTokenGenerator {
        InstallationTokenGenerator {
            installation_id,
            jwt_credential: Box::new(crate::Credentials::JWT(creds)),
            access_key: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) fn token(&self) -> Option<String> {
        if let crate::Credentials::JWT(ref creds) = *self.jwt_credential {
            if creds.is_stale() {
                return None;
            }
        }
        self.access_key.lock().unwrap().clone()
    }

    pub(crate) fn jwt(&self) -> &crate::Credentials {
        &*self.jwt_credential
    }
}

impl PartialEq for InstallationTokenGenerator {
    fn eq(&self, other: &InstallationTokenGenerator) -> bool {
        self.installation_id == other.installation_id && self.jwt_credential == other.jwt_credential
    }
}
