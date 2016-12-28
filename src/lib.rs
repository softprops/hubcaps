//! Hubcaps provides a set of building blocks for interacting with the Github API

#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serializable_enum;
#[macro_use]
extern crate log;
#[macro_use]
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

use serde::de::Deserialize;
pub mod comments;
pub mod review_comments;
pub mod pull_commits;
pub mod keys;
pub mod gists;
pub mod deployments;
pub mod errors;
pub mod hooks;
pub mod issues;
pub mod labels;
pub mod rep;
pub mod releases;
pub mod repositories;
pub mod statuses;
pub mod pulls;
pub mod search;
pub mod organizations;

pub use rep::*;
pub use errors::Error;
use gists::{Gists, UserGists};
use search::Search;
use hyper::Client;
use hyper::client::RequestBuilder;
use hyper::method::Method;
use hyper::header::{Accept, Authorization, ContentLength, UserAgent};
use hyper::status::StatusCode;
use repositories::{Repository, Repositories, UserRepositories, OrganizationRepositories};
use organizations::{Organizations, UserOrganizations};
use std::fmt;
use std::io::Read;
use url::Url;
use std::collections::HashMap;

header! { (Link, "Link") => [String] }

const DEFAULT_HOST: &'static str = "https://api.github.com";

/// alias for Result that infers hubcaps::Error as Err
pub type Result<T> = std::result::Result<T, Error>;

/// enum representation of github pull and issue state
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    /// Only open issues
    Open,
    /// Only closed issues
    Closed,
    /// All issues, open or closed
    All,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   State::Open => "open",
                   State::Closed => "closed",
                   State::All => "all",
               })
    }
}

impl Default for State {
    fn default() -> State {
        State::Open
    }
}

/// enum representation of Github list sorting options
#[derive(Clone, Debug, PartialEq)]
pub enum SortDirection {
    /// Sort in ascending order
    Asc,
    /// Sort in descending order
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   SortDirection::Asc => "asc",
                   SortDirection::Desc => "desc",
               })
    }
}

impl Default for SortDirection {
    fn default() -> SortDirection {
        SortDirection::Asc
    }
}

/// Various forms of authentication credentials supported by Github
#[derive(Debug, PartialEq)]
pub enum Credentials {
    /// No authentication
    None,
    /// Oauth token string
    /// https://developer.github.com/v3/#oauth2-token-sent-in-a-header
    Token(String),
    /// Oauth client id and secret
    /// https://developer.github.com/v3/#oauth2-keysecret
    Client(String, String),
}

impl Default for Credentials {
    fn default() -> Credentials {
        Credentials::None
    }
}

/// Entry point interface for interacting with Github API
pub struct Github<'a> {
    host: String,
    agent: String,
    client: &'a Client,
    credentials: Credentials,
}

impl<'a> Github<'a> {
    /// Create a new Github instance
    pub fn new<A>(agent: A, client: &'a Client, credentials: Credentials) -> Github<'a>
        where A: Into<String>
    {
        Github::host(DEFAULT_HOST, agent, client, credentials)
    }

    /// Create a new Github instance hosted at a custom location.
    /// Useful for github enterprise installations ( yourdomain.com/api/v3/ )
    pub fn host<H, A>(host: H, agent: A, client: &'a Client, credentials: Credentials) -> Github<'a>
        where H: Into<String>,
              A: Into<String>
    {
        Github {
            host: host.into(),
            agent: agent.into(),
            client: client,
            credentials: credentials,
        }
    }

    /// Return a reference to a Github reposistory
    pub fn repo<O, R>(&self, owner: O, repo: R) -> Repository
        where O: Into<String>,
              R: Into<String>
    {
        Repository::new(self, owner, repo)
    }

    /// Return a reference to the collection of repositories owned by and
    /// associated with an owner
    pub fn user_repos<S>(&self, owner: S) -> UserRepositories
        where S: Into<String>
    {
        UserRepositories::new(self, owner)
    }

    /// Return a reference to the collection of repositories owned by the user
    /// associated with the current authentication credentials
    pub fn repos(&self) -> Repositories {
        Repositories::new(self)
    }

    /// Return a reference to the collection of organizations that the user
    /// associated with the current authentication credentials is in
    pub fn orgs(&self) -> Organizations {
        Organizations::new(self)
    }

    /// Return a reference to the collection of organizations a user
    /// is publicly associated with
    pub fn user_orgs<U>(&self, user: U) -> UserOrganizations
        where U: Into<String>
    {
        UserOrganizations::new(self, user)
    }

    /// Return a reference to an interface that provides access to a user's gists
    pub fn user_gists<O>(&self, owner: O) -> UserGists
        where O: Into<String>
    {
        UserGists::new(self, owner)
    }

    /// Return a reference to an interface that provides access to the
    /// gists belonging to the owner of the token used to configure this client
    pub fn gists(&self) -> Gists {
        Gists::new(self)
    }

    pub fn search(&self) -> Search {
        Search::new(self)
    }

    /// Return a reference to the collection of repositories owned by and
    /// associated with an organization
    pub fn org_repos<O>(&self, org: O) -> OrganizationRepositories
        where O: Into<String>
    {
        OrganizationRepositories::new(self, org)
    }

    fn authenticate(&self, method: Method, url: String) -> RequestBuilder {
        match self.credentials {
            Credentials::Token(ref token) => {
                self.client.request(method, &url).header(Authorization(format!("token {}", token)))
            }
            Credentials::Client(ref id, ref secret) => {

                let mut parsed = Url::parse(&url).unwrap();
                parsed.query_pairs_mut()
                    .append_pair("client_id", id)
                    .append_pair("client_secret", secret);
                self.client.request(method, parsed)
            }
            Credentials::None => self.client.request(method, &url),
        }
    }


    fn iter<D, I>(&'a self, uri: String, into_items: fn(D) -> Vec<I>) -> Result<Iter<'a, D, I>>
        where D: Deserialize
    {
        Iter::new(self, self.host.clone() + &uri, into_items)
    }

    fn request<D>(&self,
                  method: Method,
                  uri: String,
                  body: Option<&'a [u8]>)
                  -> Result<(Option<Links>, D)>
        where D: Deserialize
    {
        let builder = self.authenticate(method, uri)
            .header(UserAgent(self.agent.to_owned()))
            // todo: parameterize media type
            .header(Accept(vec!["application/vnd.github.v3+json".parse().unwrap()]));

        let mut res = try!(match body {
            Some(ref bod) => builder.body(*bod).send(),
            _ => builder.send(),
        });

        let mut body = match res.headers.clone().get::<ContentLength>() {
            Some(&ContentLength(len)) => String::with_capacity(len as usize),
            _ => String::new(),
        };
        try!(res.read_to_string(&mut body));

        let links = res.headers.get::<Link>().map(|&Link(ref value)| Links::new(value.to_owned()));

        debug!("rec response {:#?} {:#?} {}", res.status, res.headers, body);
        match res.status {
            StatusCode::Conflict |
            StatusCode::BadRequest |
            StatusCode::UnprocessableEntity |
            StatusCode::Unauthorized |
            StatusCode::NotFound |
            StatusCode::Forbidden => {
                Err(Error::Fault {
                    code: res.status,
                    error: try!(serde_json::from_str::<ClientError>(&body)),
                })
            }
            _ => Ok((links, try!(serde_json::from_str::<D>(&body)))),
        }
    }

    fn request_entity<D>(&self, method: Method, uri: String, body: Option<&'a [u8]>) -> Result<D>
        where D: Deserialize
    {
        self.request(method, uri, body).map(|(_, entity)| entity)
    }

    fn get<D>(&self, uri: &str) -> Result<D>
        where D: Deserialize
    {
        self.request_entity(Method::Get, self.host.clone() + uri, None)
    }

    fn delete(&self, uri: &str) -> Result<()> {
        match self.request_entity::<()>(Method::Delete, self.host.clone() + uri, None) {
            Err(Error::Codec(_)) => Ok(()),
            otherwise => otherwise,
        }
    }

    fn post<D>(&self, uri: &str, message: &[u8]) -> Result<D>
        where D: Deserialize
    {
        self.request_entity(Method::Post, self.host.clone() + uri, Some(message))
    }

    fn patch<D>(&self, uri: &str, message: &[u8]) -> Result<D>
        where D: Deserialize
    {
        self.request_entity(Method::Patch, self.host.clone() + uri, Some(message))
    }

    fn put<D>(&self, uri: &str, message: &[u8]) -> Result<D>
        where D: Deserialize
    {
        self.request_entity(Method::Put, self.host.clone() + uri, Some(message))
    }
}

pub struct Iter<'a, D, I> {
    github: &'a Github<'a>,
    next_link: Option<String>,
    into_items: fn(D) -> Vec<I>,
    items: Vec<I>,
}

impl<'a, D, I> Iter<'a, D, I>
    where D: Deserialize
{
    pub fn new(github: &'a Github<'a>,
               uri: String,
               into_items: fn(D) -> Vec<I>)
               -> Result<Iter<'a, D, I>> {
        let (links, payload) = try!(github.request::<D>(Method::Get, uri, None));
        Ok(Iter {
            github: github,
            next_link: links.and_then(|l| l.next()),
            into_items: into_items,
            items: into_items(payload),
        })
    }

    fn set_next(&mut self, next: Option<String>) {
        self.next_link = next;
    }
}

impl<'a, D, I> Iterator for Iter<'a, D, I>
    where D: Deserialize
{
    type Item = I;
    fn next(&mut self) -> Option<I> {
        self.items.pop().or_else(|| {
            match self.next_link.clone() {
                None => None,
                Some(ref next_link) => {
                    match self.github
                        .request::<D>(Method::Get, next_link.to_owned(), None) {
                        Ok((links, payload)) => {
                            self.set_next(links.and_then(|l| l.next()));
                            self.items = (self.into_items)(payload);
                            self.next()
                        }
                        _ => None,
                    }
                }
            }
        })
    }
}

#[derive(Debug)]
pub struct Links {
    values: HashMap<String, String>,
}

impl Links {
    pub fn new<V>(value: V) -> Links
        where V: Into<String>
    {
        let values = value.into()
            .split(",")
            .map(|link| {
                let parts = link.split(";").collect::<Vec<_>>();
                (parts[1].to_owned().replace(" rel=\"", "").replace("\"", ""),
                 parts[0].to_owned().replace("<", "").replace(">", "").replace(" ", ""))
            })
            .fold(HashMap::new(), |mut acc, (rel, link)| {
                acc.insert(rel, link);
                acc
            });
        Links { values: values }
    }

    pub fn next(&self) -> Option<String> {
        self.values.get("next").map(|s| s.to_owned())
    }

    pub fn prev(&self) -> Option<String> {
        self.values.get("prev").map(|s| s.to_owned())
    }

    pub fn last(&self) -> Option<String> {
        self.values.get("last").map(|s| s.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_links() {
        let links = Links::new(r#"<linknext>; rel="next", <linklast>; rel="last""#);
        assert_eq!(links.next(), Some("linknext".to_owned()));
        assert_eq!(links.last(), Some("linklast".to_owned()));
    }

    #[test]
    fn default_state() {
        let default: State = Default::default();
        assert_eq!(default, State::Open)
    }

    #[test]
    fn default_sort_direction() {
        let default: SortDirection = Default::default();
        assert_eq!(default, SortDirection::Asc)
    }

    #[test]
    fn default_credentials() {
        let default: Credentials = Default::default();
        assert_eq!(default, Credentials::None)
    }
}
