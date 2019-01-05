//! Traffic interface
use std::fmt;

use hyper::client::connect::Connect;
use serde::Deserialize;

use crate::{Future, Github};

/// Describes types of breakdowns of the data for views or clones
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimeUnit {
    Week,
    Day,
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TimeUnit::Week => "week",
            TimeUnit::Day => "day",
        }
        .fmt(f)
    }
}

/// Provides access to the traffic information for a repository
pub struct Traffic<C>
where
    C: Clone + Connect + 'static,
{
    github: Github<C>,
    owner: String,
    repo: String,
}

impl<C: Clone + Connect + 'static> Traffic<C> {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github<C>, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Traffic {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/traffic{}", self.owner, self.repo, more)
    }

    /// List the top 10 referrers over the past 14 days
    pub fn referrers(&self) -> Future<Vec<Referrer>> {
        self.github.get(&self.path("/popular/referrers"))
    }

    /// List the top 10 popular contents over the past 14 days
    pub fn paths(&self) -> Future<Vec<Path>> {
        self.github.get(&self.path("/popular/paths"))
    }

    /// Return the total number of views and breakdown per day or week for the last 14 days
    pub fn views(&self, unit: TimeUnit) -> Future<Views> {
        let path = match unit {
            TimeUnit::Week => "/views?per=week",
            TimeUnit::Day => "/views?per=day",
        };
        self.github.get(&self.path(path))
    }

    /// Return the total number of clones and breakdown per day or week for the last 14 days
    pub fn clones(&self, unit: TimeUnit) -> Future<Clones> {
        let path = match unit {
            TimeUnit::Week => "/clones?per=week",
            TimeUnit::Day => "/clones?per=day",
        };
        self.github.get(&self.path(path))
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Referrer {
    pub referrer: String,
    pub count: u32,
    pub uniques: u32,
}

#[derive(Debug, Deserialize)]
pub struct Path {
    pub path: String,
    pub title: String,
    pub count: u32,
    pub uniques: u32,
}

#[derive(Debug, Deserialize)]
pub struct Views {
    pub count: u32,
    pub uniques: u32,
    pub views: Vec<DataPoint>,
}

#[derive(Debug, Deserialize)]
pub struct Clones {
    pub count: u32,
    pub uniques: u32,
    pub clones: Vec<DataPoint>,
}

#[derive(Debug, Deserialize)]
pub struct DataPoint {
    pub timestamp: String,
    pub count: u32,
    pub uniques: u32,
}
