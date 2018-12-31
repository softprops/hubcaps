//! Labels interface

use hyper::client::connect::Connect;
use serde_json;

use {unfold, Future, Github, Stream};

fn identity<T>(x: T) -> T {
    x
}

pub struct Labels<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> Labels<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
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
        unfold(
            self.github.clone(),
            self.github.get_pages(&self.path("")),
            identity,
        )
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
