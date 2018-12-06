//! Labels interface

use hyper::client::connect::Connect;

use self::super::{AuthenticationConstraint, Future, Github, MediaType};

pub struct App<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

impl<C> App<C>
where
    C: Clone + Connect + 'static,
{
    #[doc(hidden)]
    pub(crate) fn new(github: Github<C>) -> Self {
        App { github: github }
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
        R: Into<String> {
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
