//! Repository interface
extern crate futures;
extern crate serde_json;

use std::collections::HashMap;
use std::fmt;

use hyper::client::connect::Connect;
use url::{form_urlencoded, Url};

use branches::Branches;
use checks::CheckRuns;
use content::Content;
use deployments::Deployments;
use git::Git;
use hooks::Hooks;
use issues::{IssueRef, Issues};
use keys::Keys;
use labels::Labels;
use pulls::PullRequests;
use releases::Releases;
use statuses::Statuses;
use teams::RepoTeams;
use traffic::Traffic;
use users::Contributors;
use users::User;
use {unfold, Future, Github, SortDirection, Stream};

fn identity<T>(x: T) -> T {
    x
}

/// describes repository visibilities
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Visibility {
    All,
    Public,
    Private,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Visibility::All => "all",
            Visibility::Public => "public",
            Visibility::Private => "private",
        }
        .fmt(f)
    }
}

/// Describes sorting options for repositories
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sort {
    Created,
    Updated,
    Pushed,
    FullName,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sort::Created => "created",
            Sort::Updated => "updated",
            Sort::Pushed => "pushed",
            Sort::FullName => "full_name",
        }
        .fmt(f)
    }
}

/// Describes member affiliation types for repositories
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Affiliation {
    Owner,
    Collaborator,
    OrganizationMember,
}

impl fmt::Display for Affiliation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Affiliation::Owner => "owner",
            Affiliation::Collaborator => "collaborator",
            Affiliation::OrganizationMember => "organization_member",
        }
        .fmt(f)
    }
}

/// Describes types of repositories
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    All,
    Owner,
    Public,
    Private,
    Member,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::All => "all",
            Type::Owner => "owner",
            Type::Public => "public",
            Type::Private => "private",
            Type::Member => "member",
        }
        .fmt(f)
    }
}

/// Describes types of organization repositories
#[derive(Clone, Copy, Debug, PartialEq)]
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
        match *self {
            OrgRepoType::All => "all",
            OrgRepoType::Public => "public",
            OrgRepoType::Private => "private",
            OrgRepoType::Forks => "forks",
            OrgRepoType::Sources => "sources",
            OrgRepoType::Member => "member",
        }
        .fmt(f)
    }
}

#[derive(Clone)]
pub struct Repositories<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

impl<C: Clone + Connect + 'static> Repositories<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }

    fn path(&self, more: &str) -> String {
        format!("/user/repos{}", more)
    }

    /// Create a new repository
    /// https://developer.github.com/v3/repos/#create
    pub fn create(&self, repo: &RepoOptions) -> Future<Repo> {
        self.github.post(&self.path(""), json!(repo))
    }

    /// list the authenticated users repositories
    /// https://developer.github.com/v3/repos/#list-your-repositories
    pub fn list(&self, options: &RepoListOptions) -> Future<Vec<Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// provides a stream over all pages of the authenticated users repositories
    /// https://developer.github.com/v3/repos/#list-your-repositories
    pub fn iter(&self, options: &RepoListOptions) -> Stream<Repo> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        unfold(
            self.github.clone(),
            self.github.get_pages(&uri.join("?")),
            identity,
        )
    }
}

/// Provides access to the authenticated user's repositories
pub struct OrgRepositories<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    org: String,
}

impl<C: Clone + Connect + 'static> OrgRepositories<C> {
    #[doc(hidden)]
    pub fn new<O>(github: Github<C>, org: O) -> Self
    where
        O: Into<String>,
    {
        OrgRepositories {
            github,
            org: org.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/orgs/{}/repos{}", self.org, more)
    }

    /// https://developer.github.com/v3/repos/#list-organization-repositories
    pub fn list(&self, options: &OrgRepoListOptions) -> Future<Vec<Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// provides a stream over all pages of an orgs's repositories
    /// https://developer.github.com/v3/repos/#list-organization-repositories
    pub fn iter(&self, options: &OrgRepoListOptions) -> Stream<Repo> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        unfold(
            self.github.clone(),
            self.github.get_pages(&uri.join("?")),
            identity,
        )
    }

    /// Create a new org repository
    /// https://developer.github.com/v3/repos/#create
    pub fn create(&self, repo: &RepoOptions) -> Future<Repo> {
        self.github.post(&self.path(""), json!(repo))
    }
}

/// Provides access to the authenticated user's repositories
pub struct UserRepositories<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
}

impl<C: Clone + Connect + 'static> UserRepositories<C> {
    #[doc(hidden)]
    pub fn new<O>(github: Github<C>, owner: O) -> Self
    where
        O: Into<String>,
    {
        UserRepositories {
            github,
            owner: owner.into(),
        }
    }

    fn uri(&self, options: &UserRepoListOptions) -> String {
        let mut uri = ["/users/", &self.owner, "/repos"].concat();
        if let Some(query) = options.serialize() {
            uri.push('?');
            uri.push_str(&query);
        }
        uri
    }

    /// https://developer.github.com/v3/repos/#list-user-repositories
    pub fn list(&self, options: &UserRepoListOptions) -> Future<Vec<Repo>> {
        self.github.get(&self.uri(options))
    }

    /// provides a stream over all pages of a user's repositories
    /// https://developer.github.com/v3/repos/#list-your-repositories
    pub fn iter(&self, options: &UserRepoListOptions) -> Stream<Repo> {
        unfold(
            self.github.clone(),
            self.github.get_pages(&self.uri(options)),
            identity,
        )
    }
}

/// Provides access to an organization's repositories
pub struct OrganizationRepositories<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    org: String,
}

impl<C: Clone + Connect + 'static> OrganizationRepositories<C> {
    #[doc(hidden)]
    pub fn new<O>(github: Github<C>, org: O) -> Self
    where
        O: Into<String>,
    {
        OrganizationRepositories {
            github,
            org: org.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/orgs/{}/repos{}", self.org, more)
    }

    /// list an organization's repositories
    /// https://developer.github.com/v3/repos/#list-organization-repositories
    pub fn list(&self, options: &OrganizationRepoListOptions) -> Future<Vec<Repo>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// Provides a stream over all pages of an organization's repositories
    /// https://developer.github.com/v3/repos/#list-organization-repositories
    pub fn iter(&self, options: &OrganizationRepoListOptions) -> Stream<Repo> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        unfold(
            self.github.clone(),
            self.github.get_pages(&uri.join("?")),
            identity,
        )
    }
}

pub struct Repository<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> Repository<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Repository {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}{}", self.owner, self.repo, more)
    }

    /// get a reference to the GitHub repository object that this `Repository` refers to
    pub fn get(&self) -> Future<Repo> {
        self.github.get(&self.path(""))
    }

    /// https://developer.github.com/v3/repos/#edit
    pub fn edit(&self, options: &RepoEditOptions) -> Future<Repo> {
        // Note that this intentionally calls POST rather than PATCH,
        // even though the docs say PATCH.
        // In my tests (changing the default branch) POST works while PATCH doesn't.
        self.github.post(&self.path(""), json!(options))
    }

    /// https://developer.github.com/v3/repos/#delete-a-repository
    pub fn delete(&self) -> Future<()> {
        self.github.delete(&self.path(""))
    }

    /// get a reference to branch operations
    pub fn branches(&self) -> Branches<C> {
        Branches::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to content operations
    pub fn content(&self) -> Content<C> {
        Content::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to git operations
    pub fn git(&self) -> Git<C> {
        Git::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to repo hook operations
    pub fn hooks(&self) -> Hooks<C> {
        Hooks::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [deployments](https://developer.github.com/v3/repos/deployments/)
    /// associated with this repository ref
    pub fn deployments(&self) -> Deployments<C> {
        Deployments::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to a specific github issue associated with this repository ref
    pub fn issue(&self, number: u64) -> IssueRef<C> {
        IssueRef::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            number,
        )
    }

    /// get a reference to github issues associated with this repository ref
    pub fn issues(&self) -> Issues<C> {
        Issues::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to github checks associated with this repository ref
    pub fn checkruns(&self) -> CheckRuns<C> {
        CheckRuns::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [deploy keys](https://developer.github.com/v3/repos/keys/)
    /// associated with this repository ref
    pub fn keys(&self) -> Keys<C> {
        Keys::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a list of labels associated with this repository ref
    pub fn labels(&self) -> Labels<C> {
        Labels::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a list of [pulls](https://developer.github.com/v3/pulls/)
    /// associated with this repository ref
    pub fn pulls(&self) -> PullRequests<C> {
        PullRequests::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [releases](https://developer.github.com/v3/repos/releases/)
    /// associated with this repository ref
    pub fn releases(&self) -> Releases<C> {
        Releases::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [statuses](https://developer.github.com/v3/repos/statuses/)
    /// associated with this repository ref
    pub fn statuses(&self) -> Statuses<C> {
        Statuses::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to [teams](https://developer.github.com/v3/repos/#list-teams)
    /// associated with this repository ref
    pub fn teams(&self) -> RepoTeams<C> {
        RepoTeams::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference to
    /// [contributors](https://developer.github.com/v3/repos/#list-contributors)
    /// associated with this repository ref
    pub fn contributors(&self) -> Contributors<C> {
        Contributors::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }

    /// get a reference of [traffic](https://developer.github.com/v3/repos/traffic/)
    /// associated with this repository ref
    pub fn traffic(&self) -> Traffic<C> {
        Traffic::new(self.github.clone(), self.owner.as_str(), self.repo.as_str())
    }
}

// representations (todo: replace with derive_builder)

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
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))] // shippied public API
    pub fn languages<C>(&self, github: Github<C>) -> Future<HashMap<String, i64>>
    where
        C: Clone + Connect + 'static,
    {
        let url = Url::parse(&self.languages_url).unwrap();
        let uri: String = url.path().into();
        github.get(&uri)
    }
}

#[derive(Debug, Default, Serialize)]
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

pub struct RepoOptionsBuilder(RepoOptions);

impl RepoOptionsBuilder {
    pub(crate) fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        RepoOptionsBuilder(RepoOptions {
            name: name.into(),
            ..Default::default()
        })
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

    pub fn has_wiki(&mut self, has_wiki: bool) -> &mut Self {
        self.0.has_wiki = Some(has_wiki);
        self
    }

    pub fn has_downloads(&mut self, has_downloads: bool) -> &mut Self {
        self.0.has_downloads = Some(has_downloads);
        self
    }

    pub fn team_id(&mut self, team_id: i32) -> &mut Self {
        self.0.team_id = Some(team_id);
        self
    }

    pub fn auto_init(&mut self, auto_init: bool) -> &mut Self {
        self.0.auto_init = Some(auto_init);
        self
    }

    pub fn gitignore_template<GI>(&mut self, gitignore_template: GI) -> &mut Self
    where
        GI: Into<String>,
    {
        self.0.gitignore_template = Some(gitignore_template.into());
        self
    }

    pub fn license_template<L>(&mut self, license_template: L) -> &mut Self
    where
        L: Into<String>,
    {
        self.0.license_template = Some(license_template.into());
        self
    }

    pub fn build(&self) -> RepoOptions {
        RepoOptions::new(
            self.0.name.as_str(),
            self.0.description.clone(),
            self.0.homepage.clone(),
            self.0.private,
            self.0.has_issues,
            self.0.has_wiki,
            self.0.has_downloads,
            self.0.team_id,
            self.0.auto_init,
            self.0.gitignore_template.clone(),
            self.0.license_template.clone(),
        )
    }
}

impl RepoOptions {
    #![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))] // exempted
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
            private,
            has_issues,
            has_wiki,
            has_downloads,
            team_id,
            auto_init,
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
        RepoListOptionsBuilder::default()
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
pub struct RepoListOptionsBuilder(RepoListOptions);

impl RepoListOptionsBuilder {
    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn visibility(&mut self, vis: Visibility) -> &mut Self {
        self.0.params.insert("visibility", vis.to_string());
        self
    }

    pub fn affiliation(&mut self, affiliations: Vec<Affiliation>) -> &mut Self {
        self.0.params.insert(
            "affiliation",
            affiliations
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn repo_type(&mut self, tpe: Type) -> &mut Self {
        self.0.params.insert("type", tpe.to_string());
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.0.params.insert("sort", sort.to_string());
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.direction(SortDirection::Asc)
    }

    pub fn desc(&mut self) -> &mut Self {
        self.direction(SortDirection::Desc)
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut Self {
        self.0.params.insert("direction", direction.to_string());
        self
    }

    pub fn build(&self) -> RepoListOptions {
        RepoListOptions {
            params: self.0.params.clone(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct RepoEditOptions {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
}

impl RepoEditOptions {
    #![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))] // exempted
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
            private,
            has_issues,
            has_projects,
            has_wiki,
            default_branch: default_branch.map(|d| d.into()),
            allow_squash_merge,
            allow_merge_commit,
            allow_rebase_merge,
        }
    }

    pub fn builder<N: Into<String>>(name: N) -> RepoEditOptionsBuilder {
        RepoEditOptionsBuilder::new(name)
    }
}

pub struct RepoEditOptionsBuilder(RepoEditOptions);

impl RepoEditOptionsBuilder {
    pub(crate) fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        RepoEditOptionsBuilder(RepoEditOptions {
            name: name.into(),
            ..Default::default()
        })
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
        OrgRepoListOptionsBuilder::default()
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
pub struct OrgRepoListOptionsBuilder(OrgRepoListOptions);

impl OrgRepoListOptionsBuilder {
    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn repo_type(&mut self, tpe: OrgRepoType) -> &mut Self {
        self.0.params.insert("type", tpe.to_string());
        self
    }

    pub fn build(&self) -> OrgRepoListOptions {
        OrgRepoListOptions {
            params: self.0.params.clone(),
        }
    }
}

#[derive(Default)]
pub struct UserRepoListOptions {
    params: HashMap<&'static str, String>,
}

impl UserRepoListOptions {
    pub fn builder() -> UserRepoListOptionsBuilder {
        UserRepoListOptionsBuilder::default()
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
pub struct UserRepoListOptionsBuilder(UserRepoListOptions);

impl UserRepoListOptionsBuilder {
    pub fn repo_type(&mut self, tpe: Type) -> &mut Self {
        self.0.params.insert("type", tpe.to_string());
        self
    }

    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.0.params.insert("sort", sort.to_string());
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.direction(SortDirection::Asc)
    }

    pub fn desc(&mut self) -> &mut Self {
        self.direction(SortDirection::Desc)
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut Self {
        self.0.params.insert("direction", direction.to_string());
        self
    }

    pub fn build(&self) -> UserRepoListOptions {
        UserRepoListOptions {
            params: self.0.params.clone(),
        }
    }
}

#[derive(Default)]
pub struct OrganizationRepoListOptions {
    params: HashMap<&'static str, String>,
}

impl OrganizationRepoListOptions {
    pub fn builder() -> OrganizationRepoListOptionsBuilder {
        OrganizationRepoListOptionsBuilder::default()
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
pub struct OrganizationRepoListOptionsBuilder(OrganizationRepoListOptions);

impl OrganizationRepoListOptionsBuilder {
    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn repo_type(&mut self, tpe: OrgRepoType) -> &mut Self {
        self.0.params.insert("type", tpe.to_string());
        self
    }

    pub fn build(&self) -> OrganizationRepoListOptions {
        OrganizationRepoListOptions {
            params: self.0.params.clone(),
        }
    }
}
