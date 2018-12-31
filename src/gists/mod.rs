//! Gists interface

use std::collections::HashMap;
use std::hash::Hash;

use hyper::client::connect::Connect;
use url::form_urlencoded;

use users::User;
use {serde_json, Future, Github};

/// reference to gists associated with a github user
pub struct UserGists<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
}

impl<C: Clone + Connect + 'static> UserGists<C> {
    #[doc(hidden)]
    pub fn new<O>(github: Github<C>, owner: O) -> Self
    where
        O: Into<String>,
    {
        UserGists {
            github,
            owner: owner.into(),
        }
    }

    pub fn list(&self, options: &GistListOptions) -> Future<Vec<Gist>> {
        let mut uri = vec![format!("/users/{}/gists", self.owner)];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }
}

pub struct Gists<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

impl<C: Clone + Connect + 'static> Gists<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }

    fn path(&self, more: &str) -> String {
        format!("/gists{}", more)
    }

    pub fn star(&self, id: &str) -> Future<()> {
        self.github
            .put_no_response(&self.path(&format!("/{}/star", id)), Vec::new())
    }

    pub fn unstar(&self, id: &str) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}/star", id)))
    }

    pub fn fork(&self, id: &str) -> Future<Gist> {
        self.github
            .post(&self.path(&format!("/{}/forks", id)), Vec::new())
    }

    pub fn forks(&self, id: &str) -> Future<Vec<GistFork>> {
        self.github.get(&self.path(&format!("/{}/forks", id)))
    }

    pub fn delete(&self, id: &str) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}", id)))
    }

    pub fn get(&self, id: &str) -> Future<Gist> {
        self.github.get(&self.path(&format!("/{}", id)))
    }

    pub fn getrev(&self, id: &str, sha: &str) -> Future<Gist> {
        self.github.get(&self.path(&format!("/{}/{}", id, sha)))
    }

    pub fn list(&self, options: &GistListOptions) -> Future<Vec<Gist>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Gist>>(&uri.join("?"))
    }

    pub fn public(&self) -> Future<Vec<Gist>> {
        self.github.get(&self.path("/public"))
    }

    pub fn starred(&self) -> Future<Vec<Gist>> {
        self.github.get(&self.path("/starred"))
    }

    pub fn create(&self, gist: &GistOptions) -> Future<Gist> {
        self.github.post(&self.path(""), json!(gist))
    }

    pub fn edit(&self, id: &str, gist: &GistOptions) -> Future<Gist> {
        self.github
            .patch(&self.path(&format!("/{}", id)), json!(gist))
    }
}

// representations

#[derive(Default)]
pub struct GistListOptions {
    params: HashMap<&'static str, String>,
}

impl GistListOptions {
    pub fn since<T>(timestamp: T) -> GistListOptions
    where
        T: Into<String>,
    {
        let mut params = HashMap::new();
        params.insert("since", timestamp.into());
        GistListOptions { params }
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GistFile {
    pub size: u64,
    pub raw_url: String,
    pub content: Option<String>,
    #[serde(rename = "type")]
    pub content_type: String,
    pub truncated: Option<bool>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Gist {
    pub url: String,
    pub forks_url: String,
    pub commits_url: String,
    pub id: String,
    pub description: Option<String>,
    pub public: bool,
    pub owner: Option<User>,
    pub user: Option<User>,
    pub files: HashMap<String, GistFile>,
    pub truncated: bool,
    pub comments: u64,
    pub comments_url: String,
    pub html_url: String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GistFork {
    pub user: User,
    pub url: String,
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub content: String,
}

impl Content {
    pub fn new<F, C>(filename: Option<F>, content: C) -> Content
    where
        F: Into<String>,
        C: Into<String>,
    {
        Content {
            filename: filename.map(|f| f.into()),
            content: content.into(),
        }
    }
}

pub struct GistOptionsBuilder(GistOptions);

impl GistOptionsBuilder {
    pub(crate) fn new<K, V>(files: HashMap<K, V>) -> Self
    where
        K: Clone + Hash + Eq + Into<String>,
        V: Into<String>,
    {
        let mut contents = HashMap::new();
        for (k, v) in files {
            contents.insert(k.into(), Content::new(None as Option<String>, v.into()));
        }
        GistOptionsBuilder(GistOptions {
            files: contents,
            ..Default::default()
        })
    }

    pub fn description<D>(&mut self, desc: D) -> &mut Self
    where
        D: Into<String>,
    {
        self.0.description = Some(desc.into());
        self
    }

    pub fn public(&mut self, p: bool) -> &mut Self {
        self.0.public = Some(p);
        self
    }

    pub fn build(&self) -> GistOptions {
        GistOptions {
            files: self.0.files.clone(),
            description: self.0.description.clone(),
            public: self.0.public,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct GistOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub files: HashMap<String, Content>,
}

impl GistOptions {
    pub fn new<D, K, V>(desc: Option<D>, public: bool, files: HashMap<K, V>) -> GistOptions
    where
        D: Into<String>,
        K: Hash + Eq + Into<String>,
        V: Into<String>,
    {
        let mut contents = HashMap::new();
        for (k, v) in files {
            contents.insert(k.into(), Content::new(None as Option<String>, v.into()));
        }
        GistOptions {
            description: desc.map(|d| d.into()),
            public: Some(public),
            files: contents,
        }
    }

    pub fn builder<K, V>(files: HashMap<K, V>) -> GistOptionsBuilder
    where
        K: Clone + Hash + Eq + Into<String>,
        V: Into<String>,
    {
        GistOptionsBuilder::new(files)
    }
}

#[cfg(test)]
mod tests {
    use super::GistOptions;
    use serde::ser::Serialize;
    use serde_json;
    use std::collections::HashMap;

    fn test_encoding<E: Serialize>(tests: Vec<(E, &str)>) {
        for test in tests {
            match test {
                (k, v) => assert_eq!(serde_json::to_string(&k).unwrap(), v),
            }
        }
    }
    #[test]
    fn gist_reqs() {
        let mut files = HashMap::new();
        files.insert("foo", "bar");
        let tests = vec![
            (
                GistOptions::new(None as Option<String>, true, files.clone()),
                r#"{"public":true,"files":{"foo":{"content":"bar"}}}"#,
            ),
            (
                GistOptions::new(Some("desc"), true, files.clone()),
                r#"{"description":"desc","public":true,"files":{"foo":{"content":"bar"}}}"#,
            ),
        ];
        test_encoding(tests);
    }

    #[test]
    fn gist_req() {
        let mut files = HashMap::new();
        files.insert("test", "foo");
        let tests = vec![
            (
                GistOptions::builder(files.clone()).build(),
                r#"{"files":{"test":{"content":"foo"}}}"#,
            ),
            (
                GistOptions::builder(files.clone())
                    .description("desc")
                    .build(),
                r#"{"description":"desc","files":{"test":{"content":"foo"}}}"#,
            ),
            (
                GistOptions::builder(files.clone())
                    .description("desc")
                    .public(false)
                    .build(),
                r#"{"description":"desc","public":false,"files":{"test":{"content":"foo"}}}"#,
            ),
        ];
        test_encoding(tests)
    }
}
