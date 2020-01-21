//! Labels interface
use serde::{Deserialize, Serialize};

use crate::{Github, Result, Stream};

pub struct Labels {
    github: Github,
    owner: String,
    repo: String,
}

impl Labels {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Labels {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/labels{}", self.owner, self.repo, more)
    }

    pub async fn create(&self, lab: &LabelOptions) -> Result<Label> {
        self.github.post(&self.path(""), json!(lab)?).await
    }

    pub async fn update(&self, prevname: &str, lab: &LabelOptions) -> Result<Label> {
        self.github
            .patch(&self.path(&format!("/{}", prevname)), json!(lab)?)
            .await
    }

    pub async fn delete(&self, name: &str) -> Result<()> {
        self.github.delete(&self.path(&format!("/{}", name))).await
    }

    pub async fn list(&self) -> Result<Vec<Label>> {
        self.github.get(&self.path("")).await
    }

    /// provides a stream over all pages of this repo's labels
    pub async fn iter(&self) -> Stream<Label> {
        self.github.get_stream(&self.path("")).await
    }
}

// representations

#[derive(Debug, Serialize)]
pub struct LabelOptions {
    pub name: String,
    pub color: String,
}

impl LabelOptions {
    pub fn new<N, C>(name: N, color: C) -> LabelOptions
    where
        N: Into<String>,
        C: Into<String>,
    {
        LabelOptions {
            name: name.into(),
            color: color.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub url: String,
    pub name: String,
    pub color: String,
}
