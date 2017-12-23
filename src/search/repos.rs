use std::collections::HashMap;
use url::{self, form_urlencoded};

use {Github, Stream, Future, SortDirection};
use std::fmt;

use hyper::client::Connect;
use super::{Search, SearchResult};
use users::User;

#[derive(Debug, PartialEq)]
pub enum ReposSort {
    /// Sort by the number of stars
    Stars,
    ///Sort by the number of forks
    Forks,
    /// Sort by when the repo was last updated
    Updated
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

/// Provides access to search operations for repositories
/// https://developer.github.com/v3/search/#search-repositories
pub struct SearchRepos<C>
    where C: Clone + Connect
{
    search: Search<C>
}

impl<C: Clone + Connect> SearchRepos<C> {
    #[doc(hidden)]
    pub fn new(search: Search<C>) -> Self {
        Self {
            search
        }
    }

    fn search_uri<Q>(&self, q: Q, options: &SearchReposOptions) -> String
    where
        Q: Into<String>
    {
        let mut uri = vec!["/search/repositories".to_string()];
        let query_options = options.serialize().unwrap_or(String::new());
        let query = form_urlencoded::Serializer::new(query_options)
            .append_pair("q", &q.into())
            .finish();

        uri.push(query);
        uri.join("?")
    }

    pub fn iter<Q>(&self, q: Q, options: &SearchReposOptions) -> Stream<ReposItem>
    where
        Q: Into<String>
    {
        self.search.iter::<ReposItem> (&self.search_uri(q, options))
    }

    pub fn list<Q>(&self, q: Q, options: &SearchReposOptions) -> Future<SearchResult<ReposItem>>
    where
        Q: Into<String>
    {
        self.search.search::<ReposItem>(&self.search_uri(q, options))
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
        }else {
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
        SearchReposOptionsBuilder(SearchReposOptions { ..Default::default() })
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
        SearchReposOptions { params: self.0.params.clone() }
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
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub homepage: String,
    pub size: u32,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub language: String,
    pub forks_count: u32,
    pub open_issues_count: u32,
    pub master_branch: String,
    pub default_branch: String,
    pub score: f64
}

