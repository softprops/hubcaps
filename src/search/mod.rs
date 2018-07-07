//! Search interface

use std::collections::HashMap;
use std::fmt;

use hyper::client::connect::Connect;
use serde::de::DeserializeOwned;
use url::{self, form_urlencoded};

use labels::Label;
use users::User;
use {unfold, Future, Github, SortDirection, Stream};

mod repos;

pub use self::repos::*;

/// Sort directions for pull requests
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IssuesSort {
    /// Sort by time created
    Created,
    /// Sort by last updated
    Updated,
    /// Sort by number of comments
    Comments,
}

impl fmt::Display for IssuesSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IssuesSort::Comments => "comments",
            IssuesSort::Created => "created",
            IssuesSort::Updated => "updated",
        }.fmt(f)
    }
}

/// Provides access to general search operations
///
#[derive(Clone)]
pub struct Search<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
}

fn items<D>(result: SearchResult<D>) -> Vec<D>
where
    D: DeserializeOwned + 'static,
{
    result.items
}

impl<C: Clone + Connect + 'static> Search<C> {
    #[doc(hidden)]
    pub fn new(github: Github<C>) -> Self {
        Self { github }
    }

    /// return a reference to a search interface for issues
    pub fn issues(&self) -> SearchIssues<C> {
        SearchIssues::new(self.clone())
    }

    /// Return a reference to a search interface for repositories
    pub fn repos(&self) -> SearchRepos<C> {
        SearchRepos::new(self.clone())
    }

    fn iter<D>(&self, url: &str) -> Stream<D>
    where
        D: DeserializeOwned + 'static,
    {
        unfold(self.github.clone(), self.github.get_pages(url), items)
    }

    fn search<D>(&self, url: &str) -> Future<SearchResult<D>>
    where
        D: DeserializeOwned + 'static,
    {
        self.github.get(url)
    }
}

/// Provides access to issue search operations
/// https://developer.github.com/v3/search/#search-issues
pub struct SearchIssues<C>
where
    C: Clone + Connect + 'static,
{
    search: Search<C>,
}

impl<C: Clone + Connect + 'static> SearchIssues<C> {
    #[doc(hidden)]
    pub fn new(search: Search<C>) -> Self {
        Self { search }
    }

    fn search_uri<Q>(&self, q: Q, options: &SearchIssuesOptions) -> String
    where
        Q: Into<String>,
    {
        let mut uri = vec!["/search/issues".to_string()];
        let query_options = options.serialize().unwrap_or_default();
        let query = form_urlencoded::Serializer::new(query_options)
            .append_pair("q", &q.into())
            .finish();
        uri.push(query);
        uri.join("?")
    }

    /// Return a stream of search results repository query
    /// See [github docs](https://developer.github.com/v3/search/#parameters-3)
    /// for query format options
    pub fn iter<Q>(&self, q: Q, options: &SearchIssuesOptions) -> Stream<IssuesItem>
    where
        Q: Into<String>,
    {
        self.search.iter::<IssuesItem>(&self.search_uri(q, options))
    }

    /// Return the first page of search result repository query
    /// See [github docs](https://developer.github.com/v3/search/#parameters-3)
    /// for query format options
    pub fn list<Q>(&self, q: Q, options: &SearchIssuesOptions) -> Future<SearchResult<IssuesItem>>
    where
        Q: Into<String>,
    {
        self.search
            .search::<IssuesItem>(&self.search_uri(q, options))
    }
}

// representations (todo: replace with derive_builder)

#[derive(Default)]
pub struct SearchIssuesOptions {
    params: HashMap<&'static str, String>,
}

impl SearchIssuesOptions {
    pub fn builder() -> SearchIssuesOptionsBuilder {
        SearchIssuesOptionsBuilder::new()
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

/// Provides access to [search operations for issues and pull requests](https://developer.github.com/v3/search/#search-issues)
#[derive(Default)]
pub struct SearchIssuesOptionsBuilder(SearchIssuesOptions);

impl SearchIssuesOptionsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn sort(&mut self, sort: IssuesSort) -> &mut Self {
        self.0.params.insert("sort", sort.to_string());
        self
    }

    pub fn order(&mut self, direction: SortDirection) -> &mut Self {
        self.0.params.insert("order", direction.to_string());
        self
    }

    pub fn build(&self) -> SearchIssuesOptions {
        SearchIssuesOptions {
            params: self.0.params.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchResult<D> {
    pub total_count: u64,
    pub incomplete_results: bool,
    pub items: Vec<D>,
}

/// May reporesent a Github Issue or PullRequest
/// depending on the type of search
#[derive(Debug, Deserialize)]
pub struct IssuesItem {
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub comments: u64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub pull_request: Option<PullRequestInfo>,
    pub body: Option<String>,
}

impl IssuesItem {
    /// returns a tuple of (repo owner name, repo name) associated with this issue
    pub fn repo_tuple(&self) -> (String, String) {
        // split the last two elements off the repo url path
        let parsed = url::Url::parse(&self.repository_url).unwrap();
        let mut path = parsed.path().split('/').collect::<Vec<_>>();
        path.reverse();
        (path[1].to_owned(), path[0].to_owned())
    }
}

#[derive(Debug, Deserialize)]
pub struct PullRequestInfo {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
}
