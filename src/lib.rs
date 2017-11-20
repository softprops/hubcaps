//! Hubcaps provides a set of building blocks for interacting with the Github API
//!
//! # Examples
//!
//!  Typical use will require instantiation of a Github client. Which requires
//! a user agent string, a `hyper::Client`, and set of `hubcaps::Credentials`.
//!
//! The hyper client should be configured with tls.
//!
//! ```no_run
//! extern crate hubcaps;
//! extern crate hyper;
//! extern crate hyper_native_tls;
//!
//! use hubcaps::{Credentials, Github};
//! use hyper::Client;
//! use hyper::net::HttpsConnector;
//! use hyper_native_tls::NativeTlsClient;
//!
//! fn main() {
//!   let github = Github::new(
//!     String::from("user-agent-name"),
//!     Client::with_connector(
//!       HttpsConnector::new(
//!         NativeTlsClient::new().unwrap()
//!       )
//!     ),
//!     Credentials::Token(
//!       String::from("personal-access-token")
//!     )
//!   );
//! }
//! ```
//!
//! Github enterprise users will want to create a client with the
//! [Github#host](struct.Github.html#method.host) method
//!
//! Access to various services are provided via methods on instances of the `Github` type.
//!
//! The convention for executing operations typically looks like
//! `github.repo(.., ..).service().operation(OperationOptions)` where operation may be `create`,
//! `delete`, etc.

//! Services and their types are packaged under their own module namespace.
//! A service interface will provide access to operations and operations may access options types
//! that define the various parameter options available for the operation. Most operation option
//! types expose `builder()` methods for a builder oriented style of constructing options.
//!
//! # Errors
//!
//! Operations typically result in a `hubcaps::Result` Type which is an alias for Rust's
//! built-in Result with the Err Type fixed to the
//! [hubcaps::Error](errors/enum.Error.html) enum type.
//!
#![allow(missing_docs)] // todo: make this a deny eventually

extern crate futures;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate tokio_core;
#[cfg(feature = "tls")]
extern crate hyper_tls;

#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;

use std::fmt;

use futures::{Stream as StdStream, Future as StdFuture, IntoFuture};
use serde::de::DeserializeOwned;
use tokio_core::reactor::Handle;
use url::Url;
#[macro_use]
mod macros;

pub mod branches;
pub mod git;
pub mod users;
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
pub mod releases;
pub mod repositories;
pub mod statuses;
pub mod pulls;
//pub mod search;
pub mod teams;
pub mod organizations;

pub use errors::{Error, ErrorKind, Result};

use gists::{Gists, UserGists};
//use search::Search;
use hyper::client::{Connect, HttpConnector};
use hyper::Client;
use hyper::client::Request;
use hyper::Method;
use hyper::header::{qitem, Accept, Authorization, UserAgent, Link};
use hyper::mime::Mime;
use repositories::{Repositories, OrganizationRepositories, UserRepositories, Repository};
use organizations::{Organization, Organizations, UserOrganizations};
use users::Users;

/// Link header type
//header! { (Link, "Link") => [String] }

const DEFAULT_HOST: &'static str = "https://api.github.com";

/// A type alias for `Futures` that may return `travis::Errors`
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

/// A type alias for `Streams` that may result in `travis::Errors`
pub type Stream<T> = Box<StdStream<Item = T, Error = Error>>;

/// alias for Result that infers hubcaps::Error as Err
// pub type Result<T> = std::result::Result<T, Error>;
/// Github defined Media types
/// See [this doc](https://developer.github.com/v3/media/) for more for more information
#[derive(Clone, Copy)]
pub enum MediaType {
    /// Return json (the default)
    Json,
    /// Return json in preview form
    Preview(&'static str),
}

impl Default for MediaType {
    fn default() -> MediaType {
        MediaType::Json
    }
}

impl From<MediaType> for Mime {
    fn from(media: MediaType) -> Mime {
        match media {
            MediaType::Json => "application/vnd.github.v3+json".parse().unwrap(),
            MediaType::Preview(codename) => {
                format!("application/vnd.github.{}-preview+json", codename)
                    .parse()
                    .unwrap()
            }
        }
    }
}

/// enum representation of Github list sorting options
#[derive(Clone, Debug, PartialEq)]
pub enum SortDirection {
    /// Sort in ascending order (the default)
    Asc,
    /// Sort in descending order
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SortDirection::Asc => "asc",
            SortDirection::Desc => "desc",
        }.fmt(f)
    }
}

impl Default for SortDirection {
    fn default() -> SortDirection {
        SortDirection::Asc
    }
}

/// Various forms of authentication credentials supported by Github
#[derive(Debug, PartialEq, Clone)]
pub enum Credentials {
    /// No authentication (the default)
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
#[derive(Clone, Debug)]
pub struct Github<C>
where
    C: Clone + Connect,
{
    host: String,
    agent: String,
    client: Client<C>,
    credentials: Credentials,
}

#[cfg(feature = "tls")]
impl Github<HttpsConnector<HttpConnector>> {
    pub fn new<A>(agent: A, credentials: Credentials, handle: &Handle) -> Self
    where
        A: Into<String>,
    {
        Self::host(DEFAULT_HOST, agent, credentials, handle)
    }

    pub fn host<H, A>(host: H, agent: A, credentials: Credentials, handle: &Handle) -> Self
    where
        H: Into<String>,
        A: Into<String>,
    {
        let connector = HttpsConnector::new(4, handle).unwrap();
        let http = Client::configure()
            .connector(connector)
            .keep_alive(true)
            .build(handle);
        Self::custom(host, agent, credentials, http)
    }
}

impl<C> Github<C>
where
    C: Clone + Connect,
{
    pub fn custom<H, A>(host: H, agent: A, credentials: Credentials, http: Client<C>) -> Self
    where
        H: Into<String>,
        A: Into<String>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client: http,
            credentials: credentials,
        }
    }

    /// Return a reference to a Github repository
    pub fn repo<O, R>(&self, owner: O, repo: R) -> Repository<C>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Repository::new(self.clone(), owner, repo)
    }

    /// Return a reference to the collection of repositories owned by and
    /// associated with an owner
    pub fn user_repos<S>(&self, owner: S) -> UserRepositories<C>
    where
        S: Into<String>,
    {
        UserRepositories::new(self.clone(), owner)
    }

    /// Return a reference to the collection of repositories owned by the user
    /// associated with the current authentication credentials
    pub fn repos(&self) -> Repositories<C> {
        Repositories::new(self.clone())
    }

    pub fn org<O>(&self, org: O) -> Organization<C>
    where
        O: Into<String>,
    {
        Organization::new(self.clone(), org)
    }

    /// Return a reference to the collection of organizations that the user
    /// associated with the current authentication credentials is in
    pub fn orgs(&self) -> Organizations<C> {
        Organizations::new(self.clone())
    }

    /// Return a reference to an interface that provides access
    /// to user information.
    pub fn users(&self) -> Users<C> {
        Users::new(self.clone())
    }

    /// Return a reference to the collection of organizations a user
    /// is publicly associated with
    pub fn user_orgs<U>(&self, user: U) -> UserOrganizations<C>
    where
        U: Into<String>,
    {
        UserOrganizations::new(self.clone(), user)
    }

    /// Return a reference to an interface that provides access to a user's gists
    pub fn user_gists<O>(&self, owner: O) -> UserGists<C>
    where
        O: Into<String>,
    {
        UserGists::new(self.clone(), owner)
    }

    /// Return a reference to an interface that provides access to the
    /// gists belonging to the owner of the token used to configure this client
    pub fn gists(&self) -> Gists<C> {
        Gists::new(self.clone())
    }

    /*/// Return a reference to an interface that provides access to search operations
    pub fn search(&self) -> Search {
        Search::new(self)
    }*/

    /// Return a reference to the collection of repositories owned by and
    /// associated with an organization
    pub fn org_repos<O>(&self, org: O) -> OrganizationRepositories<C>
    where
        O: Into<String>,
    {
        OrganizationRepositories::new(self.clone(), org)
    }


    /*fn iter<'a, D, I>(&'a self, uri: String, into_items: fn(D) -> Vec<I>) -> Result<Iter<'a, D, I>>
    where
        D: DeserializeOwned,
    {
        self.iter_media(uri, into_items, MediaType::Json)
    }

    fn iter_media<'a, D, I>(
        &'a self,
        uri: String,
        into_items: fn(D) -> Vec<I>,
        media_type: MediaType,
    ) -> Result<Iter<'a, D, I>>
    where
        D: DeserializeOwned,
    {
        Iter::new(self, self.host.clone() + &uri, into_items, media_type)
    }*/

    fn request<Out>(
        &self,
        method: Method,
        uri: String,
        body: Option<Vec<u8>>,
        media_type: MediaType,
    ) -> Future<(Option<Link>, Out)>
    where
        Out: DeserializeOwned + 'static,
    {
        let url = if let Credentials::Client(ref id, ref secret) = self.credentials {
            let mut parsed = Url::parse(&uri).unwrap();
            parsed
                .query_pairs_mut()
                .append_pair("client_id", id)
                .append_pair("client_secret", secret);
            parsed.to_string().parse().into_future()
        } else {
            uri.parse().into_future()
        };
        let instance = self.clone();
        let response = url.map_err(Error::from).and_then(move |url| {
            let mut req = Request::new(method, url);
            {
                let headers = req.headers_mut();
                headers.set(UserAgent::new(instance.agent.clone()));
                headers.set(Accept(vec![qitem(From::from(media_type))]));
                if let Credentials::Token(ref token) = instance.credentials {
                    headers.set(Authorization(format!("token {}", token)))
                }
            }

            if let Some(body) = body {
                req.set_body(body)
            }
            instance.client.request(req).map_err(Error::from)
        });
        Box::new(response.and_then(move |response| {
            let link = response.headers().get::<Link>().map(|l| l.clone());
            let status = response.status();
            response.body().concat2().map_err(Error::from).and_then(
                move |body| {
                    if status.is_success() {
                        serde_json::from_slice::<Out>(&body)
                            .map(|out| (link, out))
                            .map_err(|error| ErrorKind::Codec(error).into())
                    } else {
                        Err(
                            ErrorKind::Fault {
                                code: status,
                                error: serde_json::from_slice(&body)?,
                            }.into(),
                        )
                    }
                },
            )
        }))
    }

    fn request_entity<D>(
        &self,
        method: Method,
        uri: String,
        body: Option<Vec<u8>>,
        media_type: MediaType,
    ) -> Future<D>
    where
        D: DeserializeOwned + 'static,
    {
        Box::new(self.request(method, uri, body, media_type).map(
            |(_, entity)| {
                entity
            },
        ))
    }

    fn get<D>(&self, uri: &str) -> Future<D>
    where
        D: DeserializeOwned + 'static,
    {
        self.get_media(uri, MediaType::Json)
    }

    fn get_media<D>(&self, uri: &str, media: MediaType) -> Future<D>
    where
        D: DeserializeOwned + 'static,
    {
        self.request_entity(Method::Get, self.host.clone() + uri, None, media)
    }

    fn delete(&self, uri: &str) -> Future<()> {
        Box::new(
            self.request_entity::<()>(
                Method::Delete,
                self.host.clone() + uri,
                None,
                MediaType::Json,
            ).or_else(|err| match err {
                    Error(ErrorKind::Codec(_), _) => Ok(()),
                    otherwise => Err(otherwise.into()),
                }),
        )
    }

    fn post<D>(&self, uri: &str, message: Vec<u8>) -> Future<D>
    where
        D: DeserializeOwned + 'static,
    {
        self.request_entity(
            Method::Post,
            self.host.clone() + uri,
            Some(message),
            MediaType::Json,
        )
    }

    fn patch_media<D>(&self, uri: &str, message: Vec<u8>, media: MediaType) -> Future<D>
    where
        D: DeserializeOwned + 'static,
    {
        self.request_entity(Method::Patch, self.host.clone() + uri, Some(message), media)
    }

    fn patch<D>(&self, uri: &str, message: Vec<u8>) -> Future<D>
    where
        D: DeserializeOwned + 'static,
    {
        self.patch_media(uri, message, MediaType::Json)
    }

    fn put_no_response(&self, uri: &str, message: Vec<u8>) -> Future<()> {
        Box::new(self.put(uri, message).or_else(|err| match err {
            Error(ErrorKind::Codec(_), _) => Ok(()),
            err => Err(err.into()),
        }))
    }

    fn put<D>(&self, uri: &str, message: Vec<u8>) -> Future<D>
    where
        D: DeserializeOwned + 'static,
    {
        self.request_entity(
            Method::Put,
            self.host.clone() + uri,
            Some(message),
            MediaType::Json,
        )
    }
}

/// An abstract type used for iterating over result sets
/*pub struct Iter<'a, D, I> {
    github: &'a Github,
    next_link: Option<String>,
    into_items: fn(D) -> Vec<I>,
    items: Vec<I>,
    media_type: MediaType,
}

impl<'a, D, I> Iter<'a, D, I>
where
    D: DeserializeOwned,
{
    /// creates a new instance of an Iter
    pub fn new(
        github: &'a Github,
        uri: String,
        into_items: fn(D) -> Vec<I>,
        media_type: MediaType,
    ) -> Result<Iter<'a, D, I>> {
        let (links, payload) = github.request::<D>(Method::Get, uri, None, media_type)?;
        let mut items = into_items(payload);
        items.reverse(); // we pop from the tail
        Ok(Iter {
            github: github,
            next_link: links.and_then(|l| l.next()),
            into_items: into_items,
            items: items,
            media_type: media_type,
        })
    }

    fn set_next(&mut self, next: Option<String>) {
        self.next_link = next;
    }
}

impl<'a, D, I> Iterator for Iter<'a, D, I>
where
    D: DeserializeOwned,
{
    type Item = I;
    fn next(&mut self) -> Option<I> {
        self.items.pop().or_else(|| {
            self.next_link.clone().and_then(|ref next_link| {
                self.github
                    .request::<D>(Method::Get, next_link.to_owned(), None, self.media_type)
                    .ok()
                    .and_then(|(links, payload)| {
                        let mut next_items = (self.into_items)(payload);
                        next_items.reverse(); // we pop() from the tail
                        self.set_next(links.and_then(|l| l.next()));
                        self.items = next_items;
                        self.next()
                    })
            })
        })
    }
}*/
/// An abstract collection of Link header urls
/// Exposes interfaces to access link relations github typically
/// sends as headers
/*#[derive(Debug)]
pub struct Links {
    values: HashMap<String, String>,
}

impl Links {
    /// Creates a new Links instance given a raw header string value
    pub fn new<V>(value: V) -> Links
    where
        V: Into<String>,
    {
        let values = value
            .into()
            .split(",")
            .map(|link| {
                let parts = link.split(";").collect::<Vec<_>>();
                (
                    parts[1].to_owned().replace(" rel=\"", "").replace("\"", ""),
                    parts[0]
                        .to_owned()
                        .replace("<", "")
                        .replace(">", "")
                        .replace(" ", ""),
                )
            })
            .fold(HashMap::new(), |mut acc, (rel, link)| {
                acc.insert(rel, link);
                acc
            });
        Links { values: values }
    }

    /// Returns next link url, when available
    pub fn next(&self) -> Option<String> {
        self.values.get("next").map(|s| s.to_owned())
    }

    /// Returns prev link url, when available
    pub fn prev(&self) -> Option<String> {
        self.values.get("prev").map(|s| s.to_owned())
    }

    /// Returns last link url, when available
    pub fn last(&self) -> Option<String> {
        self.values.get("last").map(|s| s.to_owned())
    }
}*/
#[cfg(test)]
mod tests {
    /*use super::*;

    #[test]
    fn test_parse_links() {
        let links = Links::new(r#"<linknext>; rel="next", <linklast>; rel="last""#);
        assert_eq!(links.next(), Some("linknext".to_owned()));
        assert_eq!(links.last(), Some("linklast".to_owned()));
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
    }*/
}
