//! Deploy keys interface
//!
//! This [this document](https://developer.github.com/guides/managing-deploy-keys/)
//! for motivation and use
use serde::{Deserialize, Serialize};

use crate::{Github, Result};

pub struct Keys {
    github: Github,
    owner: String,
    repo: String,
}

impl Keys {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
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

    pub async fn create(&self, key: &KeyOptions) -> Result<Key> {
        self.github.post(&self.path(""), json!(key)?).await
    }

    pub async fn list(&self) -> Result<Vec<Key>> {
        self.github.get(&self.path("")).await
    }

    pub async fn get(&self, id: u64) -> Result<Key> {
        self.github.get(&self.path(&format!("/{}", id))).await
    }

    pub async fn delete(&self, id: u64) -> Result<()> {
        self.github.delete(&self.path(&format!("/{}", id))).await
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
