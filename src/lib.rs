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
//! extern crate tokio_core;
//!
//! use tokio_core::reactor::Core;
//! use hubcaps::{Credentials, Github};
//!
//! fn main() {
//!   let mut core = Core::new().unwrap();
//!   let github = Github::new(
//!     String::from("user-agent-name"),
//!     Credentials::Token(
//!       String::from("personal-access-token")
//!     ),
//!     &core.handle()
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

use std::fmt;

use futures::{future, stream, Stream as StdStream, Future as StdFuture, IntoFuture};
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use hyper::client::{Connect, HttpConnector};
use hyper::Client;
use hyper::client::Request;
use hyper::Method;
use hyper::header::{qitem, Accept, Authorization, UserAgent, Link, RelationType};
use hyper::mime::Mime;
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
pub mod search;
pub mod teams;
pub mod organizations;

pub use errors::{Error, ErrorKind, Result};

use gists::{Gists, UserGists};
use search::Search;
use repositories::{Repositories, OrganizationRepositories, UserRepositories, Repository};
use organizations::{Organization, Organizations, UserOrganizations};
use users::Users;

const DEFAULT_HOST: &'static str = "https://api.github.com";

/// A type alias for `Futures` that may return `github::Errors`
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

/// A type alias for `Streams` that may result in `github::Errors`
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

    /// Return a reference to an interface that provides access to search operations
    pub fn search(&self) -> Search<C> {
        Search::new(self.clone())
    }

    /// Return a reference to the collection of repositories owned by and
    /// associated with an organization
    pub fn org_repos<O>(&self, org: O) -> OrganizationRepositories<C>
    where
        O: Into<String>,
    {
        OrganizationRepositories::new(self.clone(), org)
    }

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
                        debug!("{}", String::from_utf8_lossy(&body));
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

    fn get_pages<D>(&self, uri: &str) -> Future<(Option<Link>, D)>
    where
        D: DeserializeOwned + 'static,
    {
        self.request(Method::Get, self.host.clone() + uri, None, MediaType::Json)
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

fn next_link(l: Link) -> Option<String> {
    l.values()
        .into_iter()
        .find(|v| {
            v.rel().unwrap_or(&[]).get(0) == Some(&RelationType::Next)
        })
        .map(|v| v.link().to_owned())
}

/// "unfold" paginated results of a list of github entities
fn unfold<C, D, I>(
    github: Github<C>,
    first: Future<(Option<Link>, D)>,
    into_items: fn(D) -> Vec<I>,
) -> Stream<I>
where
    D: DeserializeOwned + 'static,
    I: 'static,
    C: Clone + Connect,
{
    Box::new(
        first
            .map(move |(link, payload)| {
                let mut items = into_items(payload);
                items.reverse();
                stream::unfold::<_, _, Future<(I, (Option<Link>, Vec<I>))>, _>(
                    (link, items),
                    move |(link, mut items)| match items.pop() {
                        Some(item) => Some(Box::new(future::ok((item, (link, items))))),
                        _ => {
                            link.and_then(next_link).map(|url| {
                                Box::new(
                                    github
                                        .request::<D>(
                                            Method::Get,
                                            url.to_owned(),
                                            Default::default(),
                                            Default::default(),
                                        )
                                        .map(move |(link, payload)| {
                                            let mut items = into_items(payload);
                                            items.reverse();
                                            (items.remove(0), (link, items))
                                        }),
                                ) as
                                    Future<(I, (Option<Link>, Vec<I>))>
                            })
                        }
                    },
                )
            })
            .into_stream()
            .flatten(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
