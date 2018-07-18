use std::collections::HashMap;
use url::form_urlencoded;

use std::fmt;
use {Future, SortDirection, Stream};

use super::{Search, SearchResult};
use hyper::client::Connect;
use users::User;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReposSort {
    /// Sort by the number of stars
    Stars,
    ///Sort by the number of forks
    Forks,
    /// Sort by when the repo was last updated
    Updated,
}

impl fmt::Display for ReposSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReposSort::Stars => "stars",
            ReposSort::Forks => "forks",
            ReposSort::Updated => "updated",
        }.fmt(f)
    }
}

/// Provides access to [search operations for repositories](https://developer.github.com/v3/search/#search-repositories)
///
pub struct SearchRepos<C>
where
    C: Clone + Connect,
{
    search: Search<C>,
}

impl<C: Clone + Connect> SearchRepos<C> {
    #[doc(hidden)]
    pub fn new(search: Search<C>) -> Self {
        Self { search }
    }

    fn search_uri<Q>(&self, q: Q, options: &SearchReposOptions) -> String
    where
        Q: Into<String>,
    {
        let mut uri = vec!["/search/repositories".to_string()];
        let query_options = options.serialize().unwrap_or_default();
        let query = form_urlencoded::Serializer::new(query_options)
            .append_pair("q", &q.into())
            .finish();

        uri.push(query);
        uri.join("?")
    }

    /// Return a stream of search results repository query
    /// See [github docs](https://developer.github.com/v3/search/#parameters)
    /// for query format options
    pub fn iter<Q>(&self, q: Q, options: &SearchReposOptions) -> Stream<ReposItem>
    where
        Q: Into<String>,
    {
        self.search.iter::<ReposItem>(&self.search_uri(q, options))
    }

    /// Return the first page of search result repository query
    /// See [github docs](https://developer.github.com/v3/search/#parameters)
    /// for query format options
    pub fn list<Q>(&self, q: Q, options: &SearchReposOptions) -> Future<SearchResult<ReposItem>>
    where
        Q: Into<String>,
    {
        self.search
            .search::<ReposItem>(&self.search_uri(q, options))
    }
}

#[derive(Default)]
pub struct SearchReposOptions {
    params: HashMap<&'static str, String>,
}

impl SearchReposOptions {
    pub fn builder() -> SearchReposOptionsBuilder {
        SearchReposOptionsBuilder::new()
    }

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

pub struct SearchReposOptionsBuilder(SearchReposOptions);

impl SearchReposOptionsBuilder {
    pub fn new() -> SearchReposOptionsBuilder {
        SearchReposOptionsBuilder(SearchReposOptions {
            ..Default::default()
        })
    }

    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn sort(&mut self, sort: ReposSort) -> &mut Self {
        self.0.params.insert("sort", sort.to_string());
        self
    }

    pub fn order(&mut self, direction: SortDirection) -> &mut Self {
        self.0.params.insert("order", direction.to_string());
        self
    }

    pub fn build(&self) -> SearchReposOptions {
        SearchReposOptions {
            params: self.0.params.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReposItem {
    pub id: u32,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: String,
    pub size: u32,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub language: String,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: u32,
    pub mirror_url: Option<String>,
    pub archived: bool,
    pub open_issues_count: u32,
    pub license: License,
    pub forks: u32,
    pub open_issues: u32,
    pub watchers: u32,
    pub default_branch: String,
    pub score: f64,
}

#[derive(Debug, Deserialize)]
pub struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: String,
}
