//! Search interface

use self::super::{Github, Result, Iter};
use serde::Deserialize;
use std::fmt;
use url::form_urlencoded;
use std::collections::HashMap;
use super::SortDirection;
use users::User;
use url;
use labels::Label;

/// Sort directions for pull requests
#[derive(Debug, PartialEq)]
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
        write!(f,
               "{}",
               match *self {
                   IssuesSort::Comments => "comments",
                   IssuesSort::Created => "created",
                   IssuesSort::Updated => "updated",
               })
    }
}

/// Provides access to search operations
/// https://developer.github.com/v3/search/#search-issues
pub struct Search<'a> {
    github: &'a Github,
}

fn items<D: Deserialize>(result: SearchResult<D>) -> Vec<D> {
    result.items
}

impl<'a> Search<'a> {
    pub fn new(github: &'a Github) -> Search<'a> {
        Search { github: github }
    }

    /// return a reference to a search interface for issues
    pub fn issues(&self) -> SearchIssues {
        SearchIssues::new(&self)
    }

    fn iter<D: Deserialize>(&'a self, url: &str) -> Result<Iter<'a, SearchResult<D>, D>> {
        self.github.iter(url.to_owned(), items)
    }

    fn search<D: Deserialize>(&self, url: &str) -> Result<SearchResult<D>> {
        self.github.get::<SearchResult<D>>(url)
    }
}

/// Provides access to issue search operations
/// https://developer.github.com/v3/search/#search-issues
pub struct SearchIssues<'a> {
    search: &'a Search<'a>,
}

impl<'a> SearchIssues<'a> {
    pub fn new(search: &'a Search<'a>) -> SearchIssues<'a> {
        SearchIssues { search: search }
    }

    fn search_uri<Q>(q: Q, options: &SearchIssuesOptions) -> String
        where Q: Into<String>
    {
        let mut uri = vec!["/search/issues".to_string()];
        let query_options = options.serialize().unwrap_or(String::new());
        let query =
            form_urlencoded::Serializer::new(query_options).append_pair("q", &q.into()).finish();
        uri.push(query);
        uri.join("?")
    }

    /// Returns an Iterator over pages of search results
    /// Use this interface if you wish to iterate over all items
    /// in a result set
    pub fn iter<Q>(&'a self,
                   q: Q,
                   options: &SearchIssuesOptions)
                   -> Result<Iter<'a, SearchResult<IssuesItem>, IssuesItem>>
        where Q: Into<String>
    {
        self.search.iter::<IssuesItem>(&Self::search_uri(q, options))
    }

    /// Returns a single page of search results
    pub fn list<Q>(&self, q: Q, options: &SearchIssuesOptions) -> Result<SearchResult<IssuesItem>>
        where Q: Into<String>
    {
        self.search.search::<IssuesItem>(&Self::search_uri(q, options))
    }
}

// representations

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

/// https://developer.github.com/v3/search/#search-issues
#[derive(Default)]
pub struct SearchIssuesOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl SearchIssuesOptionsBuilder {
    pub fn new() -> SearchIssuesOptionsBuilder {
        SearchIssuesOptionsBuilder { ..Default::default() }
    }

    pub fn sort(&mut self, sort: IssuesSort) -> &mut SearchIssuesOptionsBuilder {
        self.params.insert("sort", sort.to_string());
        self
    }

    pub fn order(&mut self, direction: SortDirection) -> &mut SearchIssuesOptionsBuilder {
        self.params.insert("order", direction.to_string());
        self
    }

    pub fn build(&self) -> SearchIssuesOptions {
        SearchIssuesOptions { params: self.params.clone() }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchResult<D: ::serde::Deserialize> {
    pub total_count: u64,
    pub incomplete_results: bool,
    pub items: Vec<D>,
}


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
        let parsed = url::Url::parse(&self.repository_url).unwrap();
        let mut path = parsed.path().split("/").collect::<Vec<_>>();
        path.reverse();
        (path[0].to_owned(), path[1].to_owned())
    }
}

#[derive(Debug, Deserialize)]
pub struct PullRequestInfo {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
}
