//! Rust representations of Github API data structures

use std::collections::HashMap;
use rustc_serialize::json::{Json, ToJson};
use rustc_serialize::{Decoder, Decodable, Encodable, Encoder, json};
use statuses::State;

impl Decodable for Deployment {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Deployment, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
      Ok(Deployment {
        url: try!(decoder.read_struct_field("url", 0, |decoder| Decodable::decode(decoder))),
        id: try!(decoder.read_struct_field("id", 0, |decoder| Decodable::decode(decoder))),
        sha: try!(decoder.read_struct_field("sha", 0, |decoder| Decodable::decode(decoder))),
        commit_ref: try!(decoder.read_struct_field("ref", 0, |decoder| Decodable::decode(decoder))),
        task: try!(decoder.read_struct_field("task", 0, |decoder| Decodable::decode(decoder))),
        environment: try!(decoder.read_struct_field("environment", 0, |decoder| Decodable::decode(decoder))),
        description: try!(decoder.read_struct_field("description", 0, |decoder| Decodable::decode(decoder))),
        creator: try!(decoder.read_struct_field("creator", 0, |decoder| Decodable::decode(decoder))),
        created_at: try!(decoder.read_struct_field("created_at", 0, |decoder| Decodable::decode(decoder))),
        updated_at: try!(decoder.read_struct_field("updated_at", 0, |decoder| Decodable::decode(decoder))),
        statuses_url: try!(decoder.read_struct_field("statuses_url", 0, |decoder| Decodable::decode(decoder))),
        repository_url: try!(decoder.read_struct_field("repository_url", 0, |decoder| Decodable::decode(decoder))),
      })
    })
  }
}

#[derive(Debug)]
pub struct Deployment {
  url: String,
  id: i64,
  sha: String,
  commit_ref: String,
  task: String,
//  payload: Json,
  environment: String,
  description: String,
  creator: User,
  created_at: String,
  updated_at: String,
  statuses_url: String,
  repository_url: String
}

impl Encodable for DeploymentReq {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      DeploymentReq {
        commit_ref: ref cref,
        task: ref tsk,
        auto_merge: ref amrg,
        required_contexts: ref reqctx,
        payload: ref pld,
        environment: ref env,
        description: ref desc
      } => {
        encoder.emit_struct("DeploymentReq", 1usize, |encoder| {
          try!(encoder.emit_struct_field("ref", 0usize, |encoder| cref.encode(encoder)));
          if tsk.is_some() {
            try!(encoder.emit_struct_field("task", 0usize, |encoder| tsk.encode(encoder)));
          }
          if amrg.is_some() {
            try!(encoder.emit_struct_field("auto_merge", 0usize, |encoder| amrg.encode(encoder)));
          }
          if reqctx.is_some() {
            try!(encoder.emit_struct_field("required_contexts", 0usize, |encoder| reqctx.encode(encoder)));
          }
          if pld.is_some() {
            try!(encoder.emit_struct_field("payload", 0usize, |encoder| pld.encode(encoder)));
          }
          if env.is_some() {
            try!(encoder.emit_struct_field("environment", 0usize, |encoder| env.encode(encoder)));
          }
          if desc.is_some() {
            try!(encoder.emit_struct_field("description", 0usize, |encoder| desc.encode(encoder)));
          }
          Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct DeploymentReq {
  pub commit_ref: &'static str,
  pub task: Option<&'static str>,
  pub auto_merge: Option<bool>,
  pub required_contexts: Option<Vec<&'static str>>,
  /// contents of payload should be valid JSON
  pub payload: Option<String>,
  pub environment: Option<&'static str>,
  pub description: Option<&'static str>
}

impl DeploymentReq {
  pub fn builder(commit: &'static str) -> DeploymentReqBuilder {
    DeploymentReqBuilder::new(commit)
  }
}

pub struct DeploymentReqBuilder {
  pub commit_ref: &'static str,
  pub task: Option<&'static str>,
  pub auto_merge: Option<bool>,
  pub required_contexts: Option<Vec<&'static str>>,
  pub payload: Option<Json>,
  pub environment: Option<&'static str>,
  pub description: Option<&'static str>
}

impl DeploymentReqBuilder {
  pub fn new(commit: &'static str) -> DeploymentReqBuilder {
    DeploymentReqBuilder {
      commit_ref: commit,
      task: None,
      auto_merge: None,
      required_contexts: None,
      payload: None,
      environment: None,
      description: None
    }
  }

  pub fn task(&mut self, task: &'static str) -> &mut DeploymentReqBuilder {
    self.task = Some(task);
    self
  }

  pub fn auto_merge(&mut self, auto_merge: bool) -> &mut DeploymentReqBuilder {
    self.auto_merge = Some(auto_merge);
    self
  }

  pub fn required_contexts(&mut self, ctxs: Vec<&'static str>) -> &mut DeploymentReqBuilder {
    self.required_contexts = Some(ctxs);
    self
  }

  pub fn payload<T: ToJson>(&mut self, pl: T) -> &mut DeploymentReqBuilder {
    self.payload = Some(pl.to_json());
    self
  }

  pub fn environment(&mut self, env: &'static str) -> &mut DeploymentReqBuilder {
    self.environment = Some(env);
    self
  }

  pub fn description(&mut self, desc: &'static str) -> &mut DeploymentReqBuilder {
    self.description = Some(desc);
    self
  }

  pub fn build(&self) -> DeploymentReq {
    DeploymentReq {
      commit_ref: self.commit_ref,
      task: self.task,
      auto_merge: self.auto_merge,
      required_contexts: self.required_contexts.clone(),
      payload: self.payload.clone().map(|p| p.to_string()),
      environment: self.environment,
      description: self.description
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct GistFile {
  pub size: i64,
  pub raw_url: String,
  // type: String
//  pub truncated: bool,
  pub language: Option<String>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Gist {
  pub url: String,
  pub forks_url: String,
  pub commits_url: String,
  pub id: String,
  pub description: Option<String>,
  pub public: bool,
  pub owner: User,
  pub user: Option<User>,
  pub files: HashMap<String, GistFile>,
  pub comments: i64,
  pub comments_url: String,
  pub html_url: String,
  pub git_pull_url: String,
  pub git_push_url: String,
  pub created_at: String,
  pub updated_at: String
}

#[derive(Debug, RustcDecodable)]
pub struct GistFork {
  user: User,
  url: String,
  id: String,
  created_at: String,
  updated_at: String
}

impl Encodable for Content {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      Content {
        filename: ref this_filename,
        content: ref this_content,
      } => {
        encoder.emit_struct("Content", 1usize, |encoder| {
          try!(encoder.emit_struct_field("content", 0usize, |encoder| this_content.encode(encoder)));
          if this_filename.is_some() {
            try!(encoder.emit_struct_field("filename", 0usize, |encoder| this_filename.encode(encoder)));
          }
          Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct Content {
  pub filename: Option<&'static str>,
  pub content: &'static str
}

impl Content {
  pub fn new(filename: Option<&'static str>, content: &'static str) -> Content {
    Content { filename: filename, content: content }
  }
}

impl Encodable for GistReq {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      GistReq {
        description: ref this_description,
        public: ref this_public,
        files: ref this_files
      } => {
        encoder.emit_struct("GistReq", 1usize, |encoder| {
          try!(encoder.emit_struct_field("files", 0usize, |encoder| this_files.encode(encoder)));
          if this_public.is_some() {
            try!(encoder.emit_struct_field("public", 0usize, |encoder| this_public.encode(encoder)));
          }
          if this_description.is_some() {
            try!(encoder.emit_struct_field("description", 0usize, |encoder| this_description.encode(encoder)));
          }
          Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct GistReq {
  pub description: Option<&'static str>,
  pub public: Option<bool>,
  pub files: HashMap<&'static str, Content>
}

impl GistReq {
  pub fn new(desc: Option<&'static str>, public: bool, files: HashMap<&'static str, &'static str>) -> GistReq {
    let mut contents = HashMap::new();
    for (k,v) in files {
      contents.insert(k, Content::new(None, v));
    }
    GistReq {
      description: desc,
      public: Some(public),
      files: contents
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Permissions {
  pub admin: bool,
  pub push: bool,
  pub pull: bool
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Repo {
  pub id: i64,
  pub owner: User,
  pub name: String,
  pub full_name: String,
  pub description: String,
  // private (keyword)
  pub fork: bool,
  pub url: String,
  html_url: String,
  archive_url: String,
  assignees_url: String,
  blobs_url: String,
  branches_url: String,
  clone_url: String,
  collaborators_url: String,
  comments_url: String,
  commits_url: String,
  compare_url: String,
  contents_url: String,
  contributors_url: String,
  downloads_url: String,
  events_url: String,
  forks_url: String,
  git_commits_url: String,
  git_refs_url: String,
  git_tags_url: String,
  git_url: String,
  hooks_url: String,
  issue_comment_url: String,
  issue_events_url: String,
  issues_url: String,
  keys_url: String,
  labels_url: String,
  languages_url: String,
  merges_url: String,
  milestones_url: String,
  mirror_url: Option<String>,
  notifications_url: String,
  pulls_url: String,
  releases_url: String,
  ssh_url: String,
  stargazers_url: String,
  statuses_url: String,
  subscribers_url: String,
  subscription_url: String,
  svn_url: String,
  tags_url: String,
  teams_url: String,
  trees_url: String,
  homepage: Option<String>,
  language: Option<String>,
  forks_count: i64,
  stargazers_count: i64,
  watchers_count: i64,
  size: i64,
  default_branch: String,
  open_issues_count: i64,
  has_issues: bool,
  has_wiki: bool,
  has_pages: bool,
  has_downloads: bool,
  pushed_at: String,
  created_at: String,
  updated_at: String,
//  permissions: Permissions
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct User {
  pub login: String,
  pub id: u64,
  pub avatar_url: String,
  pub gravatar_id: String,
  pub url: String,
  pub html_url: String,
  pub followers_url: String,
  following_url: String,
  gists_url: String,
  starred_url: String,
  subscriptions_url: String,
  organizations_url: String,
  repos_url: String,
  events_url: String,
  received_events_url: String,
  // type (keyword)
  site_admin: bool
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Commit {
  pub label: String,
  // ref (keyword)
  pub sha: String,
  pub user: User,
  pub repo: Option<Repo>
}

#[derive(Debug, RustcEncodable)]
pub struct LabelReq {
  pub name: &'static str,
  pub color: &'static str
}

impl LabelReq {
  pub fn new(name: &'static str, color: &'static str) -> LabelReq {
    LabelReq {
      name: name,
      color: color
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Label {
  pub url: String,
  pub name: String,
  pub color: String
}

impl Encodable for PullEdit {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      PullEdit {
        title: ref this_title,
        body: ref this_body,
        state: ref this_state
      } => {
        encoder.emit_struct("PullEdit", 1usize, |encoder| {
          if this_title.is_some() {
            try!(encoder.emit_struct_field("title", 0usize, |encoder| this_title.encode(encoder)));
          }
          if this_body.is_some() {
            try!(encoder.emit_struct_field("body", 0usize, |encoder| this_body.encode(encoder)));
          }
          if this_state.is_some() {
            try!(encoder.emit_struct_field("state", 0usize, |encoder| this_state.encode(encoder)));
          }
          Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct PullEdit {
  title: Option<&'static str>,
  body: Option<&'static str>,
  state: Option<&'static str>
}

impl PullEdit {
  pub fn new(title: Option<&'static str>, body: Option<&'static str>, state: Option<&'static str>) -> PullEdit {
    PullEdit { title: title, body: body, state: state }
  }
}

#[derive(Debug, RustcEncodable)]
pub struct PullReq {
  pub title: &'static str,
  pub head: &'static str,
  pub base: &'static str,
  pub body: Option<&'static str>
}

impl PullReq {
  pub fn new(title: &'static str, head: &'static str, base: &'static str, body: Option<&'static str>) -> PullReq {
    PullReq {
      title: title,
      head: head,
      base: base,
      body: body
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Pull {
  pub id: u64,
  pub url: String,
  pub html_url: String,
  pub diff_url: String,
  pub patch_url: String,
  pub issue_url: String,
  pub commits_url: String,
  pub review_comments_url: String,
  pub review_comment_url: String,
  pub comments_url: String,
  pub statuses_url: String,
  pub number: u64,
  pub state: String,
  pub title: String,
  pub body: String,
  pub created_at: String,
  pub updated_at: String,
  pub closed_at: Option<String>,
  pub merged_at: Option<String>,
  //pub head: Commit,
//  pub base: Commit,
  // links
  pub user: User,
  pub merge_commit_sha: Option<String>,
  pub mergeable: Option<bool>,
  pub merged_by: Option<User>,
  pub comments: Option<u64>,
  pub commits: Option<u64>,
  pub additions: Option<u64>,
  pub deletions: Option<u64>,
  pub changed_files: Option<u64>
}

#[derive(Debug, RustcEncodable)]
pub struct IssueReq {
  pub title: &'static str,
  pub body: Option<&'static str>,
  pub assignee: Option<&'static str>,
  pub milestone: Option<i64>,
  pub labels: Vec<&'static str>
}

impl IssueReq {
  pub fn new(title: &'static str, body: Option<&'static str>, assignee: Option<&'static str>,
             milestone: Option<i64>, labels: Vec<&'static str>) -> IssueReq {
    IssueReq {
      title: title,
      body: body,
      assignee: assignee,
      milestone: milestone,
      labels: labels
    }
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Issue {
  pub id: i64,
  pub url: String,
  pub labels_url: String,
  pub comments_url: String,
  pub events_url: String,
  pub html_url: String,
  pub number: i64,
  pub state: String,
  pub title: String,
  pub body: String,
  pub user: User,
  pub labels: Vec<Label>,
  pub assignee: Option<User>,
  pub locked: bool,
  pub comments: i64,
  pub closed_at: Option<String>,
  pub created_at: String,
  pub updated_at: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Asset {
  pub url: String,
  pub browser_download_url: String,
  pub id: i64,
  pub name: String,
  pub label: Option<String>,
  pub state: String,
  pub content_type: String,
  pub size: i64,
  pub download_count: i64,
  pub created_at: String,
  pub updated_at: String,
  pub uploader: User
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Release {
  pub url: String,
  pub html_url: String,
  pub assets_url: String,
  pub upload_url: String,
  pub tarball_url: String,
  pub zipball_url: String,
  pub id: i64,
  pub tag_name: String,
  pub target_commitish: String,
  pub name: String,
  pub body: String,
  pub draft: bool,
  pub prerelease: bool,
  pub created_at: String,
  pub published_at: String,
  pub author: User,
  pub assets: Vec<Asset>
}

#[derive(Debug, RustcEncodable)]
pub struct ReleaseReq {
  pub tag_name: &'static str,
  pub target_commitish: Option<&'static str>,
  pub name: Option<&'static str>,
  pub body: Option<&'static str>,
  pub draft: Option<bool>,
  pub prerelease: Option<bool>
}


/// builder interface for ReleaseReq
pub struct ReleaseBuilder {
  tag: &'static str,
  commitish: Option<&'static str>,
  name: Option<&'static str>,
  body: Option<&'static str>,
  draft: Option<bool>,
  prerelease: Option<bool>
}

impl ReleaseBuilder {
  pub fn new(tag: &'static str) -> ReleaseBuilder {
    ReleaseBuilder {
      tag: tag,
      commitish: None,
      name: None,
      body: None,
      draft: None,
      prerelease: None
    }
  }

  pub fn commitish(&mut self, commit: &'static str) -> &mut ReleaseBuilder {
    self.commitish = Some(commit);
    self
  }

  pub fn name(&mut self, name: &'static str) -> &mut ReleaseBuilder {
    self.name = Some(name);
    self
  }

  pub fn body(&mut self, body: &'static str) -> &mut ReleaseBuilder {
    self.body = Some(body);
    self
  }

  pub fn draft(&mut self, draft: bool) -> &mut ReleaseBuilder {
    self.draft = Some(draft);
    self
  }

  pub fn prerelease(&mut self, pre: bool) -> &mut ReleaseBuilder {
    self.prerelease = Some(pre);
    self
  }

  pub fn build(&self) -> ReleaseReq {
    ReleaseReq::new(self.tag, self.commitish, self.name, self.body, self.draft, self.prerelease)
  }
}

impl ReleaseReq {
  pub fn new(tag: &'static str, commit: Option<&'static str>, name: Option<&'static str>, body: Option<&'static str>, draft: Option<bool>, prerelease: Option<bool>) -> ReleaseReq {
    ReleaseReq {
      tag_name: tag,
      target_commitish: commit,
      name: name,
      body: body,
      draft: draft,
      prerelease: prerelease
    }
  }

  pub fn builder(tag: &'static str) -> ReleaseBuilder {
    ReleaseBuilder::new(tag)
  }
}


#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Status {
  created_at: String,
  updated_at: String,
  state: State,
  target_url: String,
  description: String,
  id: i64,
  url: String,
  context: String,
  creator: User
}



impl Encodable for StatusReq {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
    match *self {
      StatusReq {
        state: ref this_state,
        target_url: ref this_target_url,
        description: ref this_description,
        context: ref this_context
      } => {
        encoder.emit_struct("StatusReq", 1usize, |encoder| {
          try!(encoder.emit_struct_field("state", 0usize, |encoder| this_state.encode(encoder)));
          if this_target_url.is_some() {
            try!(encoder.emit_struct_field("target_url", 0usize, |encoder| this_target_url.encode(encoder)));
          }
          if this_description.is_some() {
            try!(encoder.emit_struct_field("description", 0usize, |encoder| this_description.encode(encoder)));
          }
          if this_context.is_some() {
            try!(encoder.emit_struct_field("context", 0usize, |encoder| this_context.encode(encoder)));
          }
          Ok(())
        })
      }
    }
  }
}

#[derive(Debug)]
pub struct StatusReq {
  state: State,
  target_url: Option<&'static str>,
  description: Option<&'static str>,
  context: Option<&'static str>
}

pub struct StatusBuilder {
  state: State,
  target_url: Option<&'static str>,
  description: Option<&'static str>,
  context: Option<&'static str>,
}

impl StatusBuilder {
  pub fn new(state: State) -> StatusBuilder {
    StatusBuilder {
      state: state,
      target_url: None,
      description: None,
      context: None
    }
  }

  pub fn target_url(&mut self, url: &'static str) -> &mut StatusBuilder {
    self.target_url = Some(url);
    self
  }

  pub fn description(&mut self, desc: &'static str) -> &mut StatusBuilder {
    self.description = Some(desc);
    self
  }

  pub fn context(&mut self, ctx: &'static str) -> &mut StatusBuilder {
    self.context = Some(ctx);
    self
  }

  pub fn build(&self) -> StatusReq {
    StatusReq::new(self.state.clone(), self.target_url, self.description, self.context)
  }
}

impl StatusReq {
  pub fn new(state: State, target_url: Option<&'static str>, descr: Option<&'static str>, context: Option<&'static str>) -> StatusReq {
    StatusReq {
      state: state,
      target_url: target_url,
      description: descr,
      context: context
    }
  }

  pub fn builder(state: State) -> StatusBuilder {
    StatusBuilder::new(state)
  }
}
