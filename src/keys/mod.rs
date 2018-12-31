//! Deploy keys interface
//!
//! This [this document](https://developer.github.com/guides/managing-deploy-keys/)
//! for motivation and use

use hyper::client::connect::Connect;
use serde_json;

use {Future, Github};

pub struct Keys<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> Keys<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Keys {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/keys{}", self.owner, self.repo, more)
    }

    pub fn create(&self, key: &KeyOptions) -> Future<Key> {
        self.github.post(&self.path(""), json!(key))
    }

    pub fn list(&self) -> Future<Vec<Key>> {
        self.github.get(&self.path(""))
    }

    pub fn get(&self, id: u64) -> Future<Key> {
        self.github.get(&self.path(&format!("/{}", id)))
    }

    pub fn delete(&self, id: u64) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}", id)))
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Key {
    pub id: u64,
    pub key: String,
    pub title: String,
    pub verified: bool,
    pub created_at: String,
    pub read_only: bool,
}

#[derive(Debug, Serialize)]
pub struct KeyOptions {
    pub title: String,
    pub key: String,
    pub read_only: bool,
}
