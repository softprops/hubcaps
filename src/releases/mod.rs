//! Releases inteface
extern crate serde_json;

use self::super::{Github, Result};
use rep::User;
// use rep::{Asset, Release, ReleaseOptions};

pub struct Assets<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    releaseid: u64,
}

impl<'a> Assets<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, releaseid: u64) -> Assets<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Assets {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            releaseid: releaseid,
        }
    }

    // todo: upload asset
    // todo: edit asset

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/releases/{}/assets{}",
                self.owner,
                self.repo,
                self.releaseid,
                more)
    }

    // todo: stream interface to download
    pub fn get(&self, id: u64) -> Result<Asset> {
        self.github.get::<Asset>(&self.path(&format!("/{}", id)))
    }

    pub fn delete(&self, id: u64) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", id)))
            .map(|_| ())
    }

    pub fn list(&self) -> Result<Vec<Asset>> {
        self.github.get::<Vec<Asset>>(&self.path(""))
    }
}

pub struct ReleaseRef<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    id: u64,
}

impl<'a> ReleaseRef<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, id: u64) -> ReleaseRef<'a>
        where O: Into<String>,
              R: Into<String>
    {
        ReleaseRef {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            id: id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/releases/{}{}",
                self.owner,
                self.repo,
                self.id,
                more)
    }

    pub fn get(&self) -> Result<Release> {
        self.github.get::<Release>(&self.path(""))
    }

    pub fn assets(&self) -> Assets {
        Assets::new(self.github,
                    self.owner.as_str(),
                    self.repo.as_str(),
                    self.id)
    }
}

pub struct Releases<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Releases<'a> {
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Releases<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Releases {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/releases{}", self.owner, self.repo, more)
    }

    pub fn create(&self, rel: &ReleaseOptions) -> Result<Release> {
        let data = try!(serde_json::to_string(&rel));
        self.github.post::<Release>(&self.path(""), data.as_bytes())
    }

    pub fn edit(&self, id: u64, rel: &ReleaseOptions) -> Result<Release> {
        let data = try!(serde_json::to_string(&rel));
        self.github.patch::<Release>(&self.path(&format!("/{}", id)), data.as_bytes())
    }

    pub fn delete(&self, id: u64) -> Result<()> {
        self.github
            .delete(&self.path(&format!("/{}", id)))
            .map(|_| ())
    }

    pub fn list(&self) -> Result<Vec<Release>> {
        self.github.get::<Vec<Release>>(&self.path(""))
    }

    pub fn get(&self, id: u64) -> ReleaseRef {
        ReleaseRef::new(self.github, self.owner.as_str(), self.repo.as_str(), id)
    }
}

// representations

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

#[derive(Debug, Serialize)]
pub struct ReleaseOptions {
    pub tag_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_commitish: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub prerelease: Option<bool>,
}

/// builder interface for ReleaseOptions
#[derive(Default)]
pub struct ReleaseOptionsBuilder {
    tag: String,
    commitish: Option<String>,
    name: Option<String>,
    body: Option<String>,
    draft: Option<bool>,
    prerelease: Option<bool>,
}

impl ReleaseOptionsBuilder {
    pub fn new<T>(tag: T) -> ReleaseOptionsBuilder
        where T: Into<String>
    {
        ReleaseOptionsBuilder { tag: tag.into(), ..Default::default() }
    }

    pub fn commitish<C>(&mut self, commit: C) -> &mut ReleaseOptionsBuilder
        where C: Into<String>
    {
        self.commitish = Some(commit.into());
        self
    }

    pub fn name<N>(&mut self, name: N) -> &mut ReleaseOptionsBuilder
        where N: Into<String>
    {
        self.name = Some(name.into());
        self
    }

    pub fn body<B>(&mut self, body: B) -> &mut ReleaseOptionsBuilder
        where B: Into<String>
    {
        self.body = Some(body.into());
        self
    }

    pub fn draft(&mut self, draft: bool) -> &mut ReleaseOptionsBuilder {
        self.draft = Some(draft);
        self
    }

    pub fn prerelease(&mut self, pre: bool) -> &mut ReleaseOptionsBuilder {
        self.prerelease = Some(pre);
        self
    }

    pub fn build(&self) -> ReleaseOptions {
        ReleaseOptions::new(self.tag.as_str(),
                            self.commitish.clone(),
                            self.name.clone(),
                            self.body.clone(),
                            self.draft,
                            self.prerelease)
    }
}

impl ReleaseOptions {
    pub fn new<T, C, N, B>(tag: T,
                           commit: Option<C>,
                           name: Option<N>,
                           body: Option<B>,
                           draft: Option<bool>,
                           prerelease: Option<bool>)
                           -> ReleaseOptions
        where T: Into<String>,
              C: Into<String>,
              N: Into<String>,
              B: Into<String>
    {
        ReleaseOptions {
            tag_name: tag.into(),
            target_commitish: commit.map(|c| c.into()),
            name: name.map(|n| n.into()),
            body: body.map(|b| b.into()),
            draft: draft,
            prerelease: prerelease,
        }
    }

    pub fn builder<T>(tag: T) -> ReleaseOptionsBuilder
        where T: Into<String>
    {
        ReleaseOptionsBuilder::new(tag)
    }
}
