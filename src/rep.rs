//! Rust representations of Github API data structures

use super::SortDirection;
use super::State as StdState;
use super::issues::Sort;
use std::collections::HashMap;
use std::hash::Hash;
use rustc_serialize::json::{Json, ToJson};
use rustc_serialize::{Decoder, Decodable, Encodable, Encoder};
use statuses::State;
use url::form_urlencoded;

#[derive(Debug, RustcDecodable)]
pub struct FieldErr {
    pub resource: String,
    pub field: String,
    pub code: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ClientError {
    pub message: String,
    pub errors: Option<Vec<FieldErr>>,
}

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
    pub url: String,
    pub id: u64,
    pub sha: String,
    pub commit_ref: String,
    pub task: String,
    // payload: Json,
    pub environment: String,
    pub description: String,
    pub creator: User,
    pub created_at: String,
    pub updated_at: String,
    pub statuses_url: String,
    pub repository_url: String,
}

impl Encodable for DeploymentOptions {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            DeploymentOptions {
        commit_ref: ref cref,
        task: ref tsk,
        auto_merge: ref amrg,
        required_contexts: ref reqctx,
        payload: ref pld,
        environment: ref env,
        description: ref desc
      } => {
                encoder.emit_struct("DeploymentOptions", 1usize, |encoder| {
                    let mut index = 0;
                    try!(encoder.emit_struct_field("ref", index, |encoder| cref.encode(encoder)));
                    if tsk.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("task",
                                                       index,
                                                       |encoder| tsk.encode(encoder)));
                    }
                    if amrg.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("auto_merge",
                                                       index,
                                                       |encoder| amrg.encode(encoder)));
                    }
                    if reqctx.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("required_contexts",
                                                       index,
                                                       |encoder| reqctx.encode(encoder)));
                    }
                    if pld.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("payload",
                                                       index,
                                                       |encoder| pld.encode(encoder)));
                    }
                    if env.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("environment",
                                                       index,
                                                       |encoder| env.encode(encoder)));
                    }
                    if desc.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("description",
                                                       index,
                                                       |encoder| desc.encode(encoder)));
                    }
                    Ok(())
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct DeploymentOptions {
    pub commit_ref: String,
    pub task: Option<String>,
    pub auto_merge: Option<bool>,
    pub required_contexts: Option<Vec<String>>,
    /// contents of payload should be valid JSON
    pub payload: Option<String>,
    pub environment: Option<String>,
    pub description: Option<String>,
}

impl DeploymentOptions {
    pub fn builder<C>(commit: C) -> DeploymentOptionsBuilder
        where C: Into<String>
    {
        DeploymentOptionsBuilder::new(commit)
    }
}

#[derive(Default)]
pub struct DeploymentOptionsBuilder {
    pub commit_ref: String,
    pub task: Option<String>,
    pub auto_merge: Option<bool>,
    pub required_contexts: Option<Vec<String>>,
    pub payload: Option<Json>,
    pub environment: Option<String>,
    pub description: Option<String>,
}

impl DeploymentOptionsBuilder {
    pub fn new<C>(commit: C) -> DeploymentOptionsBuilder
        where C: Into<String>
    {
        DeploymentOptionsBuilder { commit_ref: commit.into(), ..Default::default() }
    }

    pub fn task<T>(&mut self, task: T) -> &mut DeploymentOptionsBuilder
        where T: Into<String>
    {
        self.task = Some(task.into());
        self
    }

    pub fn auto_merge(&mut self, auto_merge: bool) -> &mut DeploymentOptionsBuilder {
        self.auto_merge = Some(auto_merge);
        self
    }

    pub fn required_contexts<C>(&mut self, ctxs: Vec<C>) -> &mut DeploymentOptionsBuilder
        where C: Into<String>
    {
        self.required_contexts = Some(ctxs.into_iter().map(|c| c.into()).collect::<Vec<String>>());
        self
    }

    pub fn payload<T: ToJson>(&mut self, pl: T) -> &mut DeploymentOptionsBuilder {
        self.payload = Some(pl.to_json());
        self
    }

    pub fn environment<E>(&mut self, env: E) -> &mut DeploymentOptionsBuilder
        where E: Into<String>
    {
        self.environment = Some(env.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut DeploymentOptionsBuilder
        where D: Into<String>
    {
        self.description = Some(desc.into());
        self
    }

    pub fn build(&self) -> DeploymentOptions {
        DeploymentOptions {
            commit_ref: self.commit_ref.clone(),
            task: self.task.clone(),
            auto_merge: self.auto_merge,
            required_contexts: self.required_contexts.clone(),
            payload: self.payload.clone().map(|p| p.to_string()),
            environment: self.environment.clone(),
            description: self.description.clone(),
        }
    }
}

#[derive(Debug, RustcDecodable)]
pub struct GistFile {
    pub size: u64,
    pub raw_url: String,
    pub content: String,
    // type: String
    pub truncated: bool,
    pub language: Option<String>,
}

#[derive(Debug, RustcDecodable)]
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
    pub comments: u64,
    pub comments_url: String,
    pub html_url: String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, RustcDecodable)]
pub struct GistFork {
    pub user: User,
    pub url: String,
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Encodable for Content {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            Content {
        filename: ref this_filename,
        content: ref this_content,
      } => {
                encoder.emit_struct("Content", 1_usize, |encoder| {
                    let mut index = 0;
                    try!(encoder.emit_struct_field("content",
                                                   index,
                                                   |encoder| this_content.encode(encoder)));
                    if this_filename.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("filename",
                                                       index,
                                                       |encoder| this_filename.encode(encoder)));
                    }

                    Ok(())
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Content {
    pub filename: Option<String>,
    pub content: String,
}

impl Content {
    pub fn new<F, C>(filename: Option<F>, content: C) -> Content
        where F: Into<String>,
              C: Into<String>
    {
        Content {
            filename: filename.map(|f| f.into()),
            content: content.into(),
        }
    }
}

impl Encodable for GistOptions {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            GistOptions {
        description: ref this_description,
        public: ref this_public,
        files: ref this_files
      } => {
                encoder.emit_struct("GistOptions", 1, |encoder| {
                    let mut index: isize = -1;
                    if this_description.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("description",
                                                       index as usize,
                                                       |encoder| this_description.encode(encoder)));
                    }
                    if this_public.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("public",
                                                       index as usize,
                                                       |encoder| this_public.encode(encoder)));
                    }
                    index += 1;
                    try!(encoder.emit_struct_field("files",
                                                   index as usize,
                                                   |encoder| this_files.encode(encoder)));
                    Ok(())
                })
            }
        }
    }
}

#[derive(Default)]
pub struct GistOptionsBuilder {
    pub description: Option<String>,
    pub public: Option<bool>,
    pub files: HashMap<String, Content>,
}

impl GistOptionsBuilder {
    pub fn new<K, V>(files: HashMap<K, V>) -> GistOptionsBuilder
        where K: Clone + Hash + Eq + Into<String>,
              V: Into<String>
    {
        let mut contents = HashMap::new();
        for (k, v) in files.into_iter() {
            contents.insert(k.into(), Content::new(None as Option<String>, v.into()));
        }
        GistOptionsBuilder { files: contents, ..Default::default() }
    }

    pub fn description<D>(&mut self, desc: D) -> &mut GistOptionsBuilder
        where D: Into<String>
    {
        self.description = Some(desc.into());
        self
    }

    pub fn public(&mut self, p: bool) -> &mut GistOptionsBuilder {
        self.public = Some(p);
        self
    }

    pub fn build(&self) -> GistOptions {
        GistOptions {
            files: self.files.clone(),
            description: self.description.clone(),
            public: self.public,
        }
    }
}

#[derive(Debug)]
pub struct GistOptions {
    pub description: Option<String>,
    pub public: Option<bool>,
    pub files: HashMap<String, Content>,
}

impl GistOptions {
    pub fn new<D, K, V>(desc: Option<D>, public: bool, files: HashMap<K, V>) -> GistOptions
        where D: Into<String>,
              K: Hash + Eq + Into<String>,
              V: Into<String>
    {
        let mut contents = HashMap::new();
        for (k, v) in files.into_iter() {
            contents.insert(k.into(), Content::new(None as Option<String>, v.into()));
        }
        GistOptions {
            description: desc.map(|d| d.into()),
            public: Some(public),
            files: contents,
        }
    }
    pub fn builder<K, V>(files: HashMap<K, V>) -> GistOptionsBuilder
        where K: Clone + Hash + Eq + Into<String>,
              V: Into<String>
    {
        GistOptionsBuilder::new(files)
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Permissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Repo {
    pub id: u64,
    pub owner: User,
    pub name: String,
    pub full_name: String,
    pub description: String,
    // private (keyword)
    pub fork: bool,
    pub url: String,
    pub html_url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub hooks_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: u64,
    pub stargazers_count: u64,
    pub watchers_count: u64,
    pub size: u64,
    pub default_branch: String,
    pub open_issues_count: u64,
    pub has_issues: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub pushed_at: String,
    pub created_at: String,
    pub updated_at: String, // permissions: Permissions
}

#[derive(Debug, RustcDecodable)]
pub struct RepoDetails {
    pub id: u64,
    pub owner: User,
    pub name: String,
    pub full_name: String, // todo
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
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    // type (keyword)
    pub site_admin: bool,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Commit {
    pub label: String,
    // ref (keyword)
    pub sha: String,
    pub user: User,
    pub repo: Option<Repo>,
}

#[derive(Debug, RustcEncodable)]
pub struct LabelOptions {
    pub name: String,
    pub color: String,
}

impl LabelOptions {
    pub fn new<N, C>(name: N, color: C) -> LabelOptions
        where N: Into<String>,
              C: Into<String>
    {
        LabelOptions {
            name: name.into(),
            color: color.into(),
        }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Label {
    pub url: String,
    pub name: String,
    pub color: String,
}

#[derive(Default)]
pub struct PullEditOptionsBuilder {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<String>,
}

impl PullEditOptionsBuilder {
    pub fn new() -> PullEditOptionsBuilder {
        PullEditOptionsBuilder { ..Default::default() }
    }

    pub fn title<T>(&mut self, title: T) -> &mut PullEditOptionsBuilder
        where T: Into<String>
    {
        self.title = Some(title.into());
        self
    }

    pub fn body<B>(&mut self, body: B) -> &mut PullEditOptionsBuilder
        where B: Into<String>
    {
        self.body = Some(body.into());
        self
    }

    pub fn state<S>(&mut self, state: S) -> &mut PullEditOptionsBuilder
        where S: Into<String>
    {
        self.state = Some(state.into());
        self
    }

    pub fn build(&self) -> PullEditOptions {
        PullEditOptions {
            title: self.title.clone(),
            body: self.body.clone(),
            state: self.state.clone(),
        }
    }
}

impl Encodable for PullEditOptions {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            PullEditOptions {
        title: ref this_title,
        body: ref this_body,
        state: ref this_state
      } => {
                encoder.emit_struct("PullEditOptions", 1usize, |encoder| {
                    let mut index: isize = -1;
                    if this_title.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("title",
                                                       index as usize,
                                                       |encoder| this_title.encode(encoder)));
                    }
                    if this_body.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("body",
                                                       index as usize,
                                                       |encoder| this_body.encode(encoder)));
                    }
                    if this_state.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("state",
                                                       index as usize,
                                                       |encoder| this_state.encode(encoder)));
                    }
                    Ok(())
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct PullEditOptions {
    title: Option<String>,
    body: Option<String>,
    state: Option<String>,
}

impl PullEditOptions {
    // todo represent state as enum
    pub fn new<T, B, S>(title: Option<T>, body: Option<B>, state: Option<S>) -> PullEditOptions
        where T: Into<String>,
              B: Into<String>,
              S: Into<String>
    {
        PullEditOptions {
            title: title.map(|t| t.into()),
            body: body.map(|b| b.into()),
            state: state.map(|s| s.into()),
        }
    }
    pub fn builder() -> PullEditOptionsBuilder {
        PullEditOptionsBuilder::new()
    }
}

#[derive(Debug, RustcEncodable)]
pub struct PullOptions {
    pub title: String,
    pub head: String,
    pub base: String,
    pub body: Option<String>,
}

impl PullOptions {
    pub fn new<T, H, BS, B>(title: T, head: H, base: BS, body: Option<B>) -> PullOptions
        where T: Into<String>,
              H: Into<String>,
              BS: Into<String>,
              B: Into<String>
    {
        PullOptions {
            title: title.into(),
            head: head.into(),
            base: base.into(),
            body: body.map(|b| b.into()),
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
    // pub head: Commit,
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
    pub changed_files: Option<u64>,
}


pub struct IssueListOptions {
    state: StdState,
    sort: Sort,
    direction: SortDirection,
    assignee: Option<String>,
    creator: Option<String>,
    mentioned: Option<String>,
    labels: Vec<String>,
    since: Option<String>,
}

impl IssueListOptions {
    pub fn builder() -> IssueListOptionsBuilder {
        IssueListOptionsBuilder::new()
    }

    pub fn serialize(&self) -> String {
        let mut params = Vec::new();
        params.push(("state", self.state.to_string()));
        params.push(("sort", self.sort.to_string()));
        params.push(("direction", self.direction.to_string()));
        if let Some(ref a) = self.assignee {
            params.push(("assignee", a.to_owned()));
        }
        if let Some(ref c) = self.creator {
            params.push(("creator", c.to_owned()));
        }
        if let Some(ref m) = self.mentioned {
            params.push(("mentioned", m.to_owned()));
        }
        if let Some(ref s) = self.since {
            params.push(("since", s.to_owned()));
        }
        if !self.labels.is_empty() {
            params.push(("labels", self.labels.join(",")));
        }
        form_urlencoded::serialize(params)
    }
}

/// a mutable issue list builder
#[derive(Default)]
pub struct IssueListOptionsBuilder {
    state: StdState,
    sort: Sort,
    direction: SortDirection,
    assignee: Option<String>,
    creator: Option<String>,
    mentioned: Option<String>,
    labels: Vec<String>,
    since: Option<String>,
}

impl IssueListOptionsBuilder {
    pub fn new() -> IssueListOptionsBuilder {
        IssueListOptionsBuilder { ..Default::default() }
    }

    pub fn state(&mut self, state: StdState) -> &mut IssueListOptionsBuilder {
        self.state = state;
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut IssueListOptionsBuilder {
        self.sort = sort;
        self
    }

    pub fn asc(&mut self) -> &mut IssueListOptionsBuilder {
        self.direction(SortDirection::Asc)
    }

    pub fn desc(&mut self) -> &mut IssueListOptionsBuilder {
        self.direction(SortDirection::Desc)
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut IssueListOptionsBuilder {
        self.direction = direction;
        self
    }

    pub fn assignee<A>(&mut self, assignee: A) -> &mut IssueListOptionsBuilder
        where A: Into<String>
    {
        self.assignee = Some(assignee.into());
        self
    }

    pub fn creator<C>(&mut self, creator: C) -> &mut IssueListOptionsBuilder
        where C: Into<String>
    {
        self.creator = Some(creator.into());
        self
    }

    pub fn mentioned<M>(&mut self, mentioned: M) -> &mut IssueListOptionsBuilder
        where M: Into<String>
    {
        self.mentioned = Some(mentioned.into());
        self
    }

    pub fn labels<L>(&mut self, labels: Vec<L>) -> &mut IssueListOptionsBuilder
        where L: Into<String>
    {
        self.labels = labels.into_iter().map(|l| l.into()).collect::<Vec<String>>();
        self
    }

    pub fn since<S>(&mut self, since: S) -> &mut IssueListOptionsBuilder
        where S: Into<String>
    {
        self.since = Some(since.into());
        self
    }

    pub fn build(&self) -> IssueListOptions {
        IssueListOptions {
            state: self.state.clone(),
            sort: self.sort.clone(),
            direction: self.direction.clone(),
            assignee: self.assignee.clone(),
            creator: self.creator.clone(),
            mentioned: self.mentioned.clone(),
            labels: self.labels.clone(),
            since: self.since.clone(),
        }
    }
}

#[derive(Debug, RustcEncodable)]
pub struct IssueOptions {
    pub title: String,
    pub body: Option<String>,
    pub assignee: Option<String>,
    pub milestone: Option<u64>,
    pub labels: Vec<String>,
}

impl IssueOptions {
    pub fn new<T, B, A, L>(title: T,
                           body: Option<B>,
                           assignee: Option<A>,
                           milestone: Option<u64>,
                           labels: Vec<L>)
                           -> IssueOptions
        where T: Into<String>,
              B: Into<String>,
              A: Into<String>,
              L: Into<String>
    {
        IssueOptions {
            title: title.into(),
            body: body.map(|b| b.into()),
            assignee: assignee.map(|a| a.into()),
            milestone: milestone,
            labels: labels.into_iter().map(|l| l.into()).collect::<Vec<String>>(),
        }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Issue {
    pub id: u64,
    pub url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub body: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub assignee: Option<User>,
    pub locked: bool,
    pub comments: u64,
    pub closed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
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

#[derive(Debug, RustcEncodable)]
pub struct ReleaseOptions {
    pub tag_name: String,
    pub target_commitish: Option<String>,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: Option<bool>,
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
        ReleaseOptions::new(self.tag.as_ref(),
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

#[derive(Debug, RustcDecodable)]
pub struct DeploymentStatus {
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: State,
    pub target_url: String,
    pub description: String,
    pub id: u64,
    pub deployment_url: String,
    pub repository_url: String,
    pub creator: User,
}

impl Encodable for DeploymentStatusOptions {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            DeploymentStatusOptions {
        state: ref this_state,
        target_url: ref this_target_url,
        description: ref this_description
      } => {
                encoder.emit_struct("DeploymentStatusOptions", 1_usize, |encoder| {
                    let mut index = 0;
                    try!(encoder.emit_struct_field("state",
                                                   index,
                                                   |encoder| this_state.encode(encoder)));
                    if this_target_url.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("target_url",
                                                       index,
                                                       |encoder| this_target_url.encode(encoder)));
                    }
                    if this_description.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("description",
                                                       index,
                                                       |encoder| this_description.encode(encoder)));
                    }
                    Ok(())
                })
            }
        }
    }
}

#[derive(Default)]
pub struct DeploymentStatusOptionsBuilder {
    state: State,
    target_url: Option<String>,
    description: Option<String>,
}

impl DeploymentStatusOptionsBuilder {
    pub fn new(state: State) -> DeploymentStatusOptionsBuilder {
        DeploymentStatusOptionsBuilder { state: state, ..Default::default() }
    }

    pub fn target_url<T>(&mut self, url: T) -> &mut DeploymentStatusOptionsBuilder
        where T: Into<String>
    {
        self.target_url = Some(url.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut DeploymentStatusOptionsBuilder
        where D: Into<String>
    {
        self.description = Some(desc.into());
        self
    }

    pub fn build(&self) -> DeploymentStatusOptions {
        DeploymentStatusOptions {
            state: self.state.clone(),
            target_url: self.target_url.clone(),
            description: self.description.clone(),
        }
    }
}

#[derive(Debug)]
pub struct DeploymentStatusOptions {
    state: State,
    target_url: Option<String>,
    description: Option<String>,
}

impl DeploymentStatusOptions {
    pub fn builder(state: State) -> DeploymentStatusOptionsBuilder {
        DeploymentStatusOptionsBuilder::new(state)
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Status {
    pub created_at: String,
    pub updated_at: String,
    pub state: State,
    pub target_url: String,
    pub description: String,
    pub id: u64,
    pub url: String,
    pub context: String,
    pub creator: User,
}

impl Encodable for StatusOptions {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            StatusOptions {
        state: ref this_state,
        target_url: ref this_target_url,
        description: ref this_description,
        context: ref this_context
      } => {
                encoder.emit_struct("StatusOptions", 1usize, |encoder| {
                    let mut index = 0;
                    try!(encoder.emit_struct_field("state",
                                                   index,
                                                   |encoder| this_state.encode(encoder)));
                    if this_target_url.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("target_url",
                                                       index,
                                                       |encoder| this_target_url.encode(encoder)));
                    }
                    if this_description.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("description",
                                                       index,
                                                       |encoder| this_description.encode(encoder)));
                    }
                    if this_context.is_some() {
                        index += 1;
                        try!(encoder.emit_struct_field("context",
                                                       index,
                                                       |encoder| this_context.encode(encoder)));
                    }
                    Ok(())
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct StatusOptions {
    state: State,
    target_url: Option<String>,
    description: Option<String>,
    context: Option<String>,
}

#[derive(Default)]
pub struct StatusBuilder {
    state: State,
    target_url: Option<String>,
    description: Option<String>,
    context: Option<String>,
}

impl StatusBuilder {
    pub fn new(state: State) -> StatusBuilder {
        StatusBuilder { state: state, ..Default::default() }
    }

    pub fn target_url<T>(&mut self, url: T) -> &mut StatusBuilder
        where T: Into<String>
    {
        self.target_url = Some(url.into());
        self
    }

    pub fn description<D>(&mut self, desc: D) -> &mut StatusBuilder
        where D: Into<String>
    {
        self.description = Some(desc.into());
        self
    }

    pub fn context<C>(&mut self, ctx: C) -> &mut StatusBuilder
        where C: Into<String>
    {
        self.context = Some(ctx.into());
        self
    }

    pub fn build(&self) -> StatusOptions {
        StatusOptions::new(self.state.clone(),
                           self.target_url.clone(),
                           self.description.clone(),
                           self.context.clone())
    }
}

impl StatusOptions {
    pub fn new<T, D, C>(state: State,
                        target_url: Option<T>,
                        descr: Option<D>,
                        context: Option<C>)
                        -> StatusOptions
        where T: Into<String>,
              D: Into<String>,
              C: Into<String>
    {
        StatusOptions {
            state: state,
            target_url: target_url.map(|t| t.into()),
            description: descr.map(|d| d.into()),
            context: context.map(|c| c.into()),
        }
    }

    pub fn builder(state: State) -> StatusBuilder {
        StatusBuilder::new(state)
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Key {
    pub id: u64,
    pub key: String,
    pub title: String,
    pub verified: bool,
    pub created_at: String,
    pub read_only: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct KeyOptions {
    pub title: String,
    pub key: String,
    pub read_only: bool,
}

#[cfg(test)]
mod tests {
    use super::super::State as StdState;
    use rustc_serialize::{json, Encodable};
    use std::collections::HashMap;
    use super::*;
    use super::super::statuses::State;

    fn test_encoding<E: Encodable>(tests: Vec<(E, &str)>) {
        for test in tests {
            match test {
                (k, v) => assert_eq!(json::encode::<E>(&k).unwrap(), v),
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
                r#"{"public":true,"files":{"foo":{"content":"bar"}}}"#
            ),
            (
                GistOptions::new(Some("desc"), true, files.clone()),
                r#"{"description":"desc","public":true,"files":{"foo":{"content":"bar"}}}"#
            )
        ];
        test_encoding(tests);
    }

    #[test]
    fn deployment_reqs() {
        let tests = vec![(DeploymentOptions::builder("test").build(),
                          r#"{"ref":"test"}"#),
                         (DeploymentOptions::builder("test").task("launchit").build(),
                          r#"{"ref":"test","task":"launchit"}"#)];
        test_encoding(tests)
    }

    #[test]
    fn deployment_status_reqs() {
        let tests = vec![
            (
                DeploymentStatusOptions::builder(State::pending).build(),
                r#"{"state":"pending"}"#
            ),
            (
                DeploymentStatusOptions::builder(State::pending).target_url("http://host.com").build(),
                r#"{"state":"pending","target_url":"http://host.com"}"#
            ),
            (
                DeploymentStatusOptions::builder(State::pending).target_url("http://host.com").description("desc").build(),
                r#"{"state":"pending","target_url":"http://host.com","description":"desc"}"#
            ),
        ];
        test_encoding(tests)
    }

    #[test]
    fn pullreq_edits() {
        let tests = vec![(PullEditOptions::builder().title("test").build(),
                          r#"{"title":"test"}"#),
                         (PullEditOptions::builder().title("test").body("desc").build(),
                          r#"{"title":"test","body":"desc"}"#),
                         (PullEditOptions::builder().state("closed").build(),
                          r#"{"state":"closed"}"#)];
        test_encoding(tests)
    }

    #[test]
    fn status_reqs() {
        let tests =
            vec![(StatusOptions::builder(State::pending).build(),
                  r#"{"state":"pending"}"#),
                 (StatusOptions::builder(State::success).target_url("http://acme.com").build(),
                  r#"{"state":"success","target_url":"http://acme.com"}"#),
                 (StatusOptions::builder(State::error).description("desc").build(),
                  r#"{"state":"error","description":"desc"}"#),
                 (StatusOptions::builder(State::failure)
                      .target_url("http://acme.com")
                      .description("desc")
                      .build(),
                  r#"{"state":"failure","target_url":"http://acme.com","description":"desc"}"#)];
        test_encoding(tests)
    }

    #[test]
    fn list_reqs() {
        fn test_serialize(tests: Vec<(IssueListOptions, &str)>) {
            for test in tests {
                match test {
                    (k, v) => assert_eq!(k.serialize(), v),
                }
            }
        }
        let tests = vec![
            (
                IssueListOptions::builder().build(),
                "state=open&sort=created&direction=asc"
            ),
            (
                IssueListOptions::builder().state(StdState::Closed).build(),
                "state=closed&sort=created&direction=asc"
             ),
            (
                IssueListOptions::builder().labels(vec!["foo", "bar"]).build(),
                "state=open&sort=created&direction=asc&labels=foo%2Cbar"
            ),
        ];
        test_serialize(tests)
    }
}
