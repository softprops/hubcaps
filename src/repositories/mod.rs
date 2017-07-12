//! Repository interface
extern crate serde_json;

use self::super::{Iter, Github, Result};
use branches::Branches;
use git::Git;
use hooks::Hooks;
use deployments::Deployments;
use keys::Keys;
use issues::{IssueRef, Issues};
use labels::Labels;
use pulls::PullRequests;
use releases::Releases;
use teams::RepoTeams;
use statuses::Statuses;
use users::User;

use std::fmt;
use super::SortDirection;
use url::form_urlencoded;
use std::collections::HashMap;
use url::Url;

fn identity<T>(x: T) -> T {
    x
}

/// describes repository visibilities
#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    All,
    Public,
    Private,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Visibility::All => "all",
                Visibility::Public => "public",
                Visibility::Private => "private",
            }
        )
    }
}

/// Describes sorting options for repositories
#[derive(Clone, Debug, PartialEq)]
pub enum Sort {
    Created,
    Updated,
    Pushed,
    FullName,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Sort::Created => "created",
                Sort::Updated => "updated",
                Sort::Pushed => "pushed",
                Sort::FullName => "full_name",
            }
        )
    }
}

/// Describes member affiliation types for repositories
#[derive(Clone, Debug, PartialEq)]
pub enum Affiliation {
    Owner,
    Collaborator,
    OrganizationMember,
}

impl fmt::Display for Affiliation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Affiliation::Owner => "owner",
                Affiliation::Collaborator => "collaborator",
                Affiliation::OrganizationMember => "organization_member",
            }
        )
    }
}

/// Describes types of repositories
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    All,
    Owner,
    Public,
    Private,
    Member,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Type::All => "all",
                Type::Owner => "owner",
                Type::Public => "public",
                Type::Private => "private",
                Type::Member => "member",
            }
        )
    }
}

/// Describes types of organization repositories
#[derive(Clone, Debug, PartialEq)]
pub enum OrgRepoType {
    All,
    Public,
    Private,
    Forks,
    Sources,
    Member,
}

impl fmt::Display for OrgRepoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                OrgRepoType::All => "all",
                OrgRepoType::Public => "public",
                OrgRepoType::Private => "private",
                OrgRepoType::Forks => "forks",
                OrgRepoType::Sources => "sources",
                OrgRepoType::Member => "member",
            }
        )
    }
}

pub struct Repositories<'a> {
    github: &'a Github,
}

impl<'a> Repositories<'a> {
    #[doc(hidden)]
    pub fn new(github: &'a Github) -> Repositories<'a> {
        Repositories { github: github }
    }

    fn path(&self, more: &str) -> String {
        format!("/user/repos{}", more)
    }

    /// Create a new repository
    /// https://developer.github.com/v3/repos/#create
    pub fn create(&self, repo: &RepoOptions) -> Result<Repo> {
        let data = serde_json::to_string(&repo)?;
        self.github.post::<Repo>(&self.path(""), data.as_bytes())
    }

    /// list the authenticated users repositories
    /// https://developer.github.com/v3/repos/#list-your-repositories
    pub fn list(&self, options: &RepoListOptions) -> Result<Vec<Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Repo>>(&uri.join("?"))
    }

    /// provides an iterator over all pages of the authenticated users repositories
    /// https://developer.github.com/v3/repos/#list-your-repositories
    pub fn iter(&self, options: &RepoListOptions) -> Result<Iter<'a, Vec<Repo>, Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.iter(uri.join("?"), identity)
    }
}

/// Provides access to the authenticated user's repositories
pub struct OrgRepositories<'a> {
    github: &'a Github,
    org: String,
}

impl<'a> OrgRepositories<'a> {
    #[doc(hidden)]
    pub fn new<O>(github: &'a Github, org: O) -> Self
    where
        O: Into<String>,
    {
        OrgRepositories {
            github: github,
            org: org.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/orgs/{}/repos{}", self.org, more)
    }

    /// https://developer.github.com/v3/repos/#list-organization-repositories
    pub fn list(&self, options: &OrgRepoListOptions) -> Result<Vec<Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Repo>>(&uri.join("?"))
    }

    /// provides an iterator over all pages of an orgs's repositories
    /// https://developer.github.com/v3/repos/#list-organization-repositories
    pub fn iter(&self, options: &OrgRepoListOptions) -> Result<Iter<'a, Vec<Repo>, Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.iter(uri.join("?"), identity)
    }

    /// Create a new org repository
    /// https://developer.github.com/v3/repos/#create
    pub fn create(&self, repo: &RepoOptions) -> Result<Repo> {
        let data = serde_json::to_string(&repo)?;
        self.github.post::<Repo>(&self.path(""), data.as_bytes())
    }
}

/// Provides access to the authenticated user's repositories
pub struct UserRepositories<'a> {
    github: &'a Github,
    owner: String,
}

impl<'a> UserRepositories<'a> {
    #[doc(hidden)]
    pub fn new<O>(github: &'a Github, owner: O) -> UserRepositories<'a>
    where
        O: Into<String>,
    {
        UserRepositories {
            github: github,
            owner: owner.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/users/{}/repos{}", self.owner, more)
    }

    /// https://developer.github.com/v3/repos/#list-user-repositories
    pub fn list(&self, options: &UserRepoListOptions) -> Result<Vec<Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Repo>>(&uri.join("?"))
    }

    /// provides an iterator over all pages of a user's repositories
    /// https://developer.github.com/v3/repos/#list-your-repositories
    pub fn iter(&self, options: &UserRepoListOptions) -> Result<Iter<'a, Vec<Repo>, Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.iter(uri.join("?"), identity)
    }
}

/// Provides access to an organization's repositories
pub struct OrganizationRepositories<'a> {
    github: &'a Github,
    org: String,
}

impl<'a> OrganizationRepositories<'a> {
    #[doc(hidden)]
    pub fn new<O>(github: &'a Github, org: O) -> OrganizationRepositories<'a>
    where
        O: Into<String>,
    {
        OrganizationRepositories {
            github: github,
            org: org.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/orgs/{}/repos{}", self.org, more)
    }

    /// list an organization's repositories
    /// https://developer.github.com/v3/repos/#list-organization-repositories
    pub fn list(&self, options: &OrganizationRepoListOptions) -> Result<Vec<Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Repo>>(&uri.join("?"))
    }

    /// Provides an iterator over all pages of an organization's repositories
    /// https://developer.github.com/v3/repos/#list-organization-repositories
    pub fn iter(&self, options: &OrganizationRepoListOptions) -> Result<Iter<Vec<Repo>, Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.iter(uri.join("?"), identity)
    }
}

pub struct Repository<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Repository<'a> {
    #[doc(hidden)]
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Repository<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Repository {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}{}", self.owner, self.repo, more)
    }

    /// get a reference to branch operations
    pub fn branches(&self) -> Branches {
        Branches::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to git operations
    pub fn git(&self) -> Git {
        Git::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to repo hook operations
    pub fn hooks(&self) -> Hooks {
        Hooks::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to the GitHub repository object that this `Respository` refers to
    pub fn get(&self) -> Result<Repo> {
        self.github.get(&self.path(""))
    }

    /// https://developer.github.com/v3/repos/#edit
    pub fn edit(&self, options: &RepoEditOptions) -> Result<Repo> {
        let data = serde_json::to_string(&options)?;
        // Note that this intentionally calls POST rather than PATCH,
        // even though the docs say PATCH.
        // In my tests (changing the default branch) POST works while PATCH doesn't.
        self.github.post::<Repo>(&&self.path(""), data.as_bytes())
    }

    /// get a reference to [deployments](https://developer.github.com/v3/repos/deployments/)
    /// associated with this repository ref
    pub fn deployments(&self) -> Deployments {
        Deployments::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to a specific github issue associated with this repoistory ref
    pub fn issue(&self, number: u64) -> IssueRef {
        IssueRef::new(self.github, self.owner.as_str(), self.repo.as_str(), number)
    }

    /// get a reference to github issues associated with this repoistory ref
    pub fn issues(&self) -> Issues {
        Issues::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [deploy keys](https://developer.github.com/v3/repos/keys/)
    /// associated with this repository ref
    pub fn keys(&self) -> Keys {
        Keys::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a list of labels associated with this repository ref
    pub fn labels(&self) -> Labels {
        Labels::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a list of [pulls](https://developer.github.com/v3/pulls/)
    /// associated with this repository ref
    pub fn pulls(&self) -> PullRequests {
        PullRequests::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [releases](https://developer.github.com/v3/repos/releases/)
    /// associated with this repository ref
    pub fn releases(&self) -> Releases {
        Releases::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [statuses](https://developer.github.com/v3/repos/statuses/)
    /// associated with this reposoitory ref
    pub fn statuses(&self) -> Statuses {
        Statuses::new(self.github, self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [teams](https://developer.github.com/v3/repos/#list-teams)
    /// associated with this repository ref
    pub fn teams(&self) -> RepoTeams {
        RepoTeams::new(self.github, self.owner.as_str(), self.repo.as_str())
    }
}


// representations

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub id: u64,
    pub owner: User,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
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
    pub deployments_url: String,
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

impl Repo {
    /// Returns a map containing the
    /// [languages](https://developer.github.com/v3/repos/#list-languages) that the repository is
    /// implemented in.
    ///
    /// The keys are the language names, and the values are the number of bytes of code written in
    /// that language.
    pub fn languages(&self, github: &Github) -> Result<HashMap<String, i64>> {
        let url = Url::parse(&self.languages_url).unwrap();
        let uri: String = url.path().into();
        github.get(&uri)
    }
}


#[derive(Debug, Serialize)]
pub struct RepoOptions {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// false by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_init: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitignore_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_template: Option<String>,
}

#[derive(Default)]
pub struct RepoOptionsBuilder {
    name: String,
    description: Option<String>,
    homepage: Option<String>,
    private: Option<bool>,
    has_issues: Option<bool>,
    has_wiki: Option<bool>,
    has_downloads: Option<bool>,
    team_id: Option<i32>,
    auto_init: Option<bool>,
    gitignore_template: Option<String>,
    license_template: Option<String>,
}

impl RepoOptionsBuilder {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        RepoOptionsBuilder {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn description<D>(&mut self, description: D) -> &mut Self
    where
        D: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    pub fn homepage<H>(&mut self, homepage: H) -> &mut Self
    where
        H: Into<String>,
    {
        self.homepage = Some(homepage.into());
        self
    }

    pub fn private(&mut self, private: bool) -> &mut Self {
        self.private = Some(private);
        self
    }

    pub fn has_issues(&mut self, has_issues: bool) -> &mut Self {
        self.has_issues = Some(has_issues);
        self
    }

    pub fn has_wiki(&mut self, has_wiki: bool) -> &mut Self {
        self.has_wiki = Some(has_wiki);
        self
    }

    pub fn has_downloads(&mut self, has_downloads: bool) -> &mut Self {
        self.has_downloads = Some(has_downloads);
        self
    }

    pub fn team_id(&mut self, team_id: i32) -> &mut Self {
        self.team_id = Some(team_id);
        self
    }

    pub fn auto_init(&mut self, auto_init: bool) -> &mut Self {
        self.auto_init = Some(auto_init);
        self
    }

    pub fn gitignore_template<GI>(&mut self, gitignore_template: GI) -> &mut Self
    where
        GI: Into<String>,
    {
        self.gitignore_template = Some(gitignore_template.into());
        self
    }

    pub fn license_template<L>(&mut self, license_template: L) -> &mut Self
    where
        L: Into<String>,
    {
        self.license_template = Some(license_template.into());
        self
    }

    pub fn build(&self) -> RepoOptions {
        RepoOptions::new(
            self.name.as_str(),
            self.description.clone(),
            self.homepage.clone(),
            self.private,
            self.has_issues,
            self.has_wiki,
            self.has_downloads,
            self.team_id,
            self.auto_init,
            self.gitignore_template.clone(),
            self.license_template.clone(),
        )
    }
}

impl RepoOptions {
    pub fn new<N, D, H, GI, L>(
        name: N,
        description: Option<D>,
        homepage: Option<H>,
        private: Option<bool>,
        has_issues: Option<bool>,
        has_wiki: Option<bool>,
        has_downloads: Option<bool>,
        team_id: Option<i32>,
        auto_init: Option<bool>,
        gitignore_template: Option<GI>,
        license_template: Option<L>,
    ) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        H: Into<String>,
        GI: Into<String>,
        L: Into<String>,
    {
        RepoOptions {
            name: name.into(),
            description: description.map(|h| h.into()),
            homepage: homepage.map(|h| h.into()),
            private: private,
            has_issues: has_issues,
            has_wiki: has_wiki,
            has_downloads: has_downloads,
            team_id: team_id,
            auto_init: auto_init,
            gitignore_template: gitignore_template.map(|gi| gi.into()),
            license_template: license_template.map(|l| l.into()),
        }
    }

    pub fn builder<N: Into<String>>(name: N) -> RepoOptionsBuilder {
        RepoOptionsBuilder::new(name)
    }
}

#[derive(Default)]
pub struct RepoListOptions {
    params: HashMap<&'static str, String>,
}

impl RepoListOptions {
    pub fn builder() -> RepoListOptionsBuilder {
        RepoListOptionsBuilder::new()
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

#[derive(Default)]
pub struct RepoListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl RepoListOptionsBuilder {
    pub fn new() -> Self {
        RepoListOptionsBuilder { ..Default::default() }
    }

    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.params.insert("per_page", n.to_string());
        self
    }

    pub fn visibility(&mut self, vis: Visibility) -> &mut Self {
        self.params.insert("visibility", vis.to_string());
        self
    }

    pub fn affiliation(&mut self, affiliations: Vec<Affiliation>) -> &mut Self {
        self.params.insert(
            "affiliation",
            affiliations
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }


    pub fn repo_type(&mut self, tpe: Sort) -> &mut Self {
        self.params.insert("type", tpe.to_string());
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.params.insert("sort", sort.to_string());
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.direction(SortDirection::Asc)
    }

    pub fn desc(&mut self) -> &mut Self {
        self.direction(SortDirection::Desc)
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut Self {
        self.params.insert("direction", direction.to_string());
        self
    }

    pub fn build(&self) -> RepoListOptions {
        RepoListOptions { params: self.params.clone() }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct RepoEditOptions {
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
}

impl RepoEditOptions {
    pub fn new<N, D, H, DB>(
        name: N,
        description: Option<D>,
        homepage: Option<H>,
        private: Option<bool>,
        has_issues: Option<bool>,
        has_projects: Option<bool>,
        has_wiki: Option<bool>,
        default_branch: Option<DB>,
        allow_squash_merge: Option<bool>,
        allow_merge_commit: Option<bool>,
        allow_rebase_merge: Option<bool>,
    ) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        H: Into<String>,
        DB: Into<String>,
    {
        RepoEditOptions {
            name: name.into(),
            description: description.map(|h| h.into()),
            homepage: homepage.map(|h| h.into()),
            private: private,
            has_issues: has_issues,
            has_projects: has_projects,
            has_wiki: has_wiki,
            default_branch: default_branch.map(|d| d.into()),
            allow_squash_merge: allow_squash_merge,
            allow_merge_commit: allow_merge_commit,
            allow_rebase_merge: allow_rebase_merge,
        }
    }

    pub fn builder<N: Into<String>>(name: N) -> RepoEditOptionsBuilder {
        RepoEditOptionsBuilder::new(name)
    }
}

pub struct RepoEditOptionsBuilder(RepoEditOptions);

impl RepoEditOptionsBuilder {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        RepoEditOptionsBuilder(
            RepoEditOptions {
                name: name.into(),
                ..Default::default()
            }
        )
    }

    pub fn description<D>(&mut self, description: D) -> &mut Self
    where
        D: Into<String>,
    {
        self.0.description = Some(description.into());
        self
    }

    pub fn homepage<H>(&mut self, homepage: H) -> &mut Self
    where
        H: Into<String>,
    {
        self.0.homepage = Some(homepage.into());
        self
    }

    pub fn private(&mut self, private: bool) -> &mut Self {
        self.0.private = Some(private);
        self
    }

    pub fn has_issues(&mut self, has_issues: bool) -> &mut Self {
        self.0.has_issues = Some(has_issues);
        self
    }

    pub fn has_projects(&mut self, has_projects: bool) -> &mut Self {
        self.0.has_projects = Some(has_projects);
        self
    }

    pub fn has_wiki(&mut self, has_wiki: bool) -> &mut Self {
        self.0.has_wiki = Some(has_wiki);
        self
    }

    pub fn default_branch<DB>(&mut self, default_branch: DB) -> &mut Self
    where
        DB: Into<String>,
    {
        self.0.default_branch = Some(default_branch.into());
        self
    }

    pub fn allow_squash_merge(&mut self, allow_squash_merge: bool) -> &mut Self {
        self.0.allow_squash_merge = Some(allow_squash_merge);
        self
    }

    pub fn allow_merge_commit(&mut self, allow_merge_commit: bool) -> &mut Self {
        self.0.allow_merge_commit = Some(allow_merge_commit);
        self
    }

    pub fn allow_rebase_merge(&mut self, allow_rebase_merge: bool) -> &mut Self {
        self.0.allow_rebase_merge = Some(allow_rebase_merge);
        self
    }

    pub fn build(&self) -> RepoEditOptions {
        RepoEditOptions::new(
            self.0.name.as_str(),
            self.0.description.clone(),
            self.0.homepage.clone(),
            self.0.private,
            self.0.has_issues,
            self.0.has_projects,
            self.0.has_wiki,
            self.0.default_branch.clone(),
            self.0.allow_squash_merge,
            self.0.allow_merge_commit,
            self.0.allow_rebase_merge,
        )
    }
}

#[derive(Default)]
pub struct OrgRepoListOptions {
    params: HashMap<&'static str, String>,
}

impl OrgRepoListOptions {
    pub fn builder() -> OrgRepoListOptionsBuilder {
        OrgRepoListOptionsBuilder::new()
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


#[derive(Default)]
pub struct OrgRepoListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl OrgRepoListOptionsBuilder {
    pub fn new() -> Self {
        OrgRepoListOptionsBuilder { ..Default::default() }
    }

    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.params.insert("per_page", n.to_string());
        self
    }

    pub fn repo_type(&mut self, tpe: OrgRepoType) -> &mut Self {
        self.params.insert("type", tpe.to_string());
        self
    }

    pub fn build(&self) -> OrgRepoListOptions {
        OrgRepoListOptions { params: self.params.clone() }
    }
}

#[derive(Default)]
pub struct UserRepoListOptions {
    params: HashMap<&'static str, String>,
}

impl UserRepoListOptions {
    pub fn builder() -> UserRepoListOptionsBuilder {
        UserRepoListOptionsBuilder::new()
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

#[derive(Default)]
pub struct UserRepoListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl UserRepoListOptionsBuilder {
    pub fn new() -> Self {
        UserRepoListOptionsBuilder { ..Default::default() }
    }

    pub fn repo_type(&mut self, tpe: Type) -> &mut Self {
        self.params.insert("type", tpe.to_string());
        self
    }

    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.params.insert("per_page", n.to_string());
        self
    }

    pub fn sort(&mut self, sort: Type) -> &mut Self {
        self.params.insert("sort", sort.to_string());
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.direction(SortDirection::Asc)
    }

    pub fn desc(&mut self) -> &mut Self {
        self.direction(SortDirection::Desc)
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut Self {
        self.params.insert("direction", direction.to_string());
        self
    }

    pub fn build(&self) -> UserRepoListOptions {
        UserRepoListOptions { params: self.params.clone() }
    }
}

#[derive(Default)]
pub struct OrganizationRepoListOptions {
    params: HashMap<&'static str, String>,
}

impl OrganizationRepoListOptions {
    pub fn builder() -> OrganizationRepoListOptionsBuilder {
        OrganizationRepoListOptionsBuilder::new()
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

#[derive(Default)]
pub struct OrganizationRepoListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl OrganizationRepoListOptionsBuilder {
    pub fn new() -> Self {
        OrganizationRepoListOptionsBuilder { ..Default::default() }
    }

    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.params.insert("per_page", n.to_string());
        self
    }

    pub fn repo_type(&mut self, tpe: OrgRepoType) -> &mut Self {
        self.params.insert("type", tpe.to_string());
        self
    }

    pub fn build(&self) -> OrganizationRepoListOptions {
        OrganizationRepoListOptions { params: self.params.clone() }
    }
}
