use self::super::{Github, Result};
use rep::{SearchIssuesOptions, SearchIssuesResult};
use std::fmt;

/// Sort directions for pull requests
#[derive(Debug, PartialEq)]
pub enum Sort {
    /// Sort by time created
    Created,
    /// Sort by last updated
    Updated,
    /// Sort by number of comments
    Comments,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Sort::Comments => "comments",
                   Sort::Created => "created",
                   Sort::Updated => "updated",
               })
    }
}

// https://developer.github.com/v3/search/#search-issues
pub struct Search<'a> {
    github: &'a Github<'a>,
}

impl<'a> Search<'a> {
    pub fn new(github: &'a Github<'a>) -> Search<'a> {
        Search { github: github }
    }

    pub fn issues(&self) -> SearchIssues {
        SearchIssues::new(&self.github)
    }
}

// https://developer.github.com/v3/search/#search-issues
pub struct SearchIssues<'a> {
    github: &'a Github<'a>,
}

impl<'a> SearchIssues<'a> {
    pub fn new(github: &'a Github<'a>) -> SearchIssues<'a> {
        SearchIssues { github: github }
    }

    pub fn list(&self, options: &SearchIssuesOptions) -> Result<SearchIssuesResult> {
        let mut uri = vec!["/search/issues".to_string()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get::<SearchIssuesResult>(&uri.join("?"))
    }
}
