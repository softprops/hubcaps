use std::collections::HashMap;

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

#[derive(Debug, RustcEncodable)]
pub struct Content {
  pub filename: Option<&'static str>,
  pub content: &'static str
}

impl Content {
  pub fn new(filename: Option<&'static str>, content: &'static str) -> Content {
    Content { filename: filename, content: content }
  }
}

#[derive(Debug, RustcEncodable)]
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Label {
  pub url: String,
  pub name: String,
  pub color: String
}

#[derive(Debug, RustcEncodable)]
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
  pub title: String
}
