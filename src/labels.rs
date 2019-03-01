//! Labels interface
use serde::{Deserialize, Serialize};

use crate::{Future, Github, Stream};

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

    pub fn create(&self, lab: &LabelOptions) -> Future<Label> {
        self.github.post(&self.path(""), json!(lab))
    }

    pub fn update(&self, prevname: &str, lab: &LabelOptions) -> Future<Label> {
        self.github
            .patch(&self.path(&format!("/{}", prevname)), json!(lab))
    }

    pub fn delete(&self, name: &str) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}", name)))
    }

    pub fn list(&self) -> Future<Vec<Label>> {
        self.github.get(&self.path(""))
    }

    /// provides a stream over all pages of this repo's labels
    pub fn iter(&self) -> Stream<Label> {
        self.github.get_stream(&self.path(""))
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
