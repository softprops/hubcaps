use self::super::{Github, Result, Iter};
use rep::{SearchIssuesOptions, SearchResult, SearchIssuesItem};
use serde::Deserialize;
use std::fmt;
use url::form_urlencoded;

/// Sort directions for pull requests
#[derive(Debug, PartialEq)]
pub enum SearchIssuesSort {
    /// Sort by time created
    Created,
    /// Sort by last updated
    Updated,
    /// Sort by number of comments
    Comments,
}

impl fmt::Display for SearchIssuesSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   SearchIssuesSort::Comments => "comments",
                   SearchIssuesSort::Created => "created",
                   SearchIssuesSort::Updated => "updated",
               })
    }
}

// https://developer.github.com/v3/search/#search-issues
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

// https://developer.github.com/v3/search/#search-issues
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

    /// returns an Iterator over pages of search results
    pub fn iter<Q>(&'a self,
                   q: Q,
                   options: &SearchIssuesOptions)
                   -> Result<Iter<'a, SearchResult<SearchIssuesItem>, SearchIssuesItem>>
        where Q: Into<String>
    {
        self.search.iter::<SearchIssuesItem>(&Self::search_uri(q, options))
    }

    /// returns a single page of search results
    pub fn list<Q>(&self,
                   q: Q,
                   options: &SearchIssuesOptions)
                   -> Result<SearchResult<SearchIssuesItem>>
        where Q: Into<String>
    {
        self.search.search::<SearchIssuesItem>(&Self::search_uri(q, options))
    }
}
