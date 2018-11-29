//! Labels interface

extern crate serde_json;

use hyper::client::connect::Connect;

use self::super::{Future, Github, MediaType};

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

    pub fn make_access_token(&self, installation_id: i32) -> Future<AccessToken> {
        self.github.post_media::<AccessToken>(
            &self.path(&format!("/installations/{}/access_tokens", installation_id)),
            Vec::new(),
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
