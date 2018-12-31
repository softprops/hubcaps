//! Releases interface
extern crate serde_json;

use hyper::client::connect::Connect;

use users::User;
use {Future, Github};

/// Provides access to assets for a release.
/// See the [github docs](https://developer.github.com/v3/repos/releases/)
/// for more information.
pub struct Assets<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
    releaseid: u64,
}

impl<C: Clone + Connect + 'static> Assets<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, releaseid: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Assets {
            github,
            owner: owner.into(),
            repo: repo.into(),
            releaseid,
        }
    }

    // todo: upload asset
    // todo: edit asset

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/releases/{}/assets{}",
            self.owner, self.repo, self.releaseid, more
        )
    }

    // todo: stream interface to download

    /// Get the asset information.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#get-a-single-release-asset)
    /// for more information.
    pub fn get(&self, id: u64) -> Future<Asset> {
        self.github.get(&self.path(&format!("/{}", id)))
    }

    /// Delete an asset by id.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#delete-a-release-asset)
    /// for more information.
    pub fn delete(&self, id: u64) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}", id)))
    }

    /// List assets for a release.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#list-assets-for-a-release)
    /// for more information.
    pub fn list(&self) -> Future<Vec<Asset>> {
        self.github.get(&self.path(""))
    }
}

pub struct ReleaseRef<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
    id: u64,
}

impl<C: Clone + Connect + 'static> ReleaseRef<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R, id: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        ReleaseRef {
            github,
            owner: owner.into(),
            repo: repo.into(),
            id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/releases/{}{}",
            self.owner, self.repo, self.id, more
        )
    }

    /// Get the release information.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#get-a-single-release)
    /// for more information.
    pub fn get(&self) -> Future<Release> {
        self.github.get::<Release>(&self.path(""))
    }

    /// Get a reference to asset operations for a release.
    pub fn assets(&self) -> Assets<C> {
        Assets::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            self.id,
        )
    }
}

/// Provides access to published releases.
/// See the [github docs](https://developer.github.com/v3/repos/releases/)
/// for more information.
pub struct Releases<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> Releases<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Releases {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/releases{}", self.owner, self.repo, more)
    }

    /// Create new a release.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#create-a-release)
    /// for more information.
    pub fn create(&self, rel: &ReleaseOptions) -> Future<Release> {
        self.github.post(&self.path(""), json!(rel))
    }

    /// Edit a release by id.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#edit-a-release)
    /// for more information.
    pub fn edit(&self, id: u64, rel: &ReleaseOptions) -> Future<Release> {
        self.github
            .patch(&self.path(&format!("/{}", id)), json!(rel))
    }

    /// Delete a release by id.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#delete-a-release)
    /// for more information.
    pub fn delete(&self, id: u64) -> Future<()> {
        self.github.delete(&self.path(&format!("/{}", id)))
    }

    /// List published releases and draft releases for users with push access.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#list-releases-for-a-repository)
    /// for more information.
    pub fn list(&self) -> Future<Vec<Release>> {
        self.github.get(&self.path(""))
    }

    /// Return the latest full release. Draft releases and prereleases are not returned.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#get-the-latest-release)
    /// for more information.
    pub fn latest(&self) -> Future<Release> {
        self.github.get(&self.path("/latest"))
    }

    /// Return a release by tag name.
    ///
    /// See the [github docs](https://developer.github.com/v3/repos/releases/#get-a-release-by-tag-name)
    /// for more information.
    pub fn by_tag<S>(&self, tag_name: S) -> Future<Release>
    where
        S: Into<String>,
    {
        self.github
            .get(&self.path(&format!("/tags/{}", tag_name.into())))
    }

    /// Get a reference to a specific release associated with a repository
    pub fn get(&self, id: u64) -> ReleaseRef<C> {
        ReleaseRef::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            id,
        )
    }
}

// representations (todo: replace with derive_builder)

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub url: String,
    pub browser_download_url: String,
    pub id: u64,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub uploader: User,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub id: u64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub author: User,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Default, Serialize)]
pub struct ReleaseOptions {
    pub tag_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
}

/// builder interface for ReleaseOptions
pub struct ReleaseOptionsBuilder(ReleaseOptions);

impl ReleaseOptionsBuilder {
    pub(crate) fn new<T>(tag: T) -> Self
    where
        T: Into<String>,
    {
        ReleaseOptionsBuilder(ReleaseOptions {
            tag_name: tag.into(),
            ..Default::default()
        })
    }

    pub fn commitish<C>(&mut self, commit: C) -> &mut Self
    where
        C: Into<String>,
    {
        self.0.target_commitish = Some(commit.into());
        self
    }

    pub fn name<N>(&mut self, name: N) -> &mut Self
    where
        N: Into<String>,
    {
        self.0.name = Some(name.into());
        self
    }

    pub fn body<B>(&mut self, body: B) -> &mut Self
    where
        B: Into<String>,
    {
        self.0.body = Some(body.into());
        self
    }

    pub fn draft(&mut self, draft: bool) -> &mut Self {
        self.0.draft = Some(draft);
        self
    }

    pub fn prerelease(&mut self, pre: bool) -> &mut Self {
        self.0.prerelease = Some(pre);
        self
    }

    pub fn build(&self) -> ReleaseOptions {
        ReleaseOptions::new(
            self.0.tag_name.as_str(),
            self.0.target_commitish.clone(),
            self.0.name.clone(),
            self.0.body.clone(),
            self.0.draft,
            self.0.prerelease,
        )
    }
}

impl ReleaseOptions {
    pub fn new<T, C, N, B>(
        tag: T,
        commit: Option<C>,
        name: Option<N>,
        body: Option<B>,
        draft: Option<bool>,
        prerelease: Option<bool>,
    ) -> Self
    where
        T: Into<String>,
        C: Into<String>,
        N: Into<String>,
        B: Into<String>,
    {
        ReleaseOptions {
            tag_name: tag.into(),
            target_commitish: commit.map(|c| c.into()),
            name: name.map(|n| n.into()),
            body: body.map(|b| b.into()),
            draft,
            prerelease,
        }
    }

    pub fn builder<T>(tag: T) -> ReleaseOptionsBuilder
    where
        T: Into<String>,
    {
        ReleaseOptionsBuilder::new(tag)
    }
}
