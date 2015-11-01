
//! Hubcaps wraps hyper with an interface for interacting with the Github API

extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub mod keys;
pub mod gists;
pub mod deployments;
pub mod errors;
pub mod issues;
pub mod labels;
pub mod rep;
pub mod releases;
pub mod repositories;
pub mod statuses;
pub mod pullrequests;

pub use rep::*;
pub use errors::Error;
use gists::{Gists, UserGists};
use hyper::Client;
use hyper::method::Method;
use hyper::header::{Authorization, ContentLength, UserAgent};
use hyper::status::StatusCode;
use repositories::{Repository, Repositories, UserRepositories};
use std::default::Default;
use std::fmt;
use std::io::Read;

/// alias for Result that infers hubcaps::Error as Err
pub type Result<T> = std::result::Result<T, Error>;

/// enum representation of github pull and issue state
pub enum State {
  Open,
  Closed,
  All
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            State::Open   => "open",
            State::Closed => "closed",
            State::All    => "all"
        })
    }
}

impl Default for State {
    fn default() -> State {
        State::Open
    }
}


/// enum representation of Github list sorting options
pub enum SortDirection {
    Asc,
    Desc
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            SortDirection::Asc => "asc",
            SortDirection::Desc => "desc"
        })
    }
}

impl Default for SortDirection {
    fn default() -> SortDirection {
        SortDirection::Asc
    }
}

/// Entry point interface for interacting with Github API
pub struct Github<'a> {
    host: &'static str,
    agent: &'static str,
    client: &'a Client,
    token: Option<&'static str>
}

impl<'a> Github<'a> {
    /// Create a new Github instance
    pub fn new(
        agent: &'static str, client: &'a Client, token: Option<&'static str>) -> Github<'a> {
        Github::host("https://api.github.com", agent, client, token)
    }

    /// Create a new Github instance hosted at a custom location
    pub fn host(
        host: &'static str,
        agent: &'static str,
        client: &'a Client,
        token: Option<&'static str>
     ) -> Github<'a> {
        Github {
            host: host,
            agent: agent,
            client: client,
            token: token
        }
    }

    /// Return a reference to a Github reposistory
    pub fn repo(&self, owner: &'static str, repo: &'static str) -> Repository {
        Repository::new(self, owner, repo)
    }

    /// Return a reference to the collection of repositories owned by an
    /// associated with an owner
    pub fn user_repos(&self, owner: &'static str) -> UserRepositories {
        UserRepositories::new(self, owner)
    }

    /// Return a reference to  the collection of repositores owned by the user
    /// associated with the current authentication credentials
    pub fn repos(&self) -> Repositories {
        Repositories::new(self)
    }

    /// Return a reference to an interface that provides access to a user's gists
    pub fn user_gists(&self, owner: &'static str) -> UserGists {
        UserGists::new(self, owner)
    }

    /// Return a reference to an interface that provides access to the
    /// gists belonging to the owner of the token used to configure this client
    pub fn gists(&self) -> Gists {
        Gists::new(self)
    }

    fn request(
        &self, method: Method, uri: &str, body: Option<&'a [u8]>) -> Result<String> {
        let url = format!("{}{}", self.host, uri);
        let builder = self.client.request(method, &url).header(
            UserAgent(self.agent.to_owned())
        );
        let authenticated = match self.token {
            Some(token) =>
                builder.header(
                    Authorization(format!("token {}", token))
               ),
            _ =>
                builder
        };
        let mut res = try!(
            match body {
                Some(ref bod) => authenticated.body(*bod).send(),
                _ => authenticated.send()
            }
        );
        let mut body =
            match res.headers.clone().get::<ContentLength>() {
                Some(&ContentLength(len)) =>
                    String::with_capacity(len as usize),
                _ => String::new()
            };
        try!(res.read_to_string(&mut body));
        match res.status {
            StatusCode::BadRequest
            | StatusCode::UnprocessableEntity
            | StatusCode::Unauthorized
            | StatusCode::NotFound
            | StatusCode::Forbidden => Err(
                Error::Fault { code: res.status, body: body }
            ),
            _ => Ok(body)
        }
    }

    fn get(&self, uri: &str) -> Result<String> {
        self.request(
            Method::Get,
            uri,
            None
        )
    }

    fn delete(&self, uri: &str) -> Result<()> {
        self.request(
            Method::Delete,
            uri,
            None
        ).map(|_| ())
    }

    fn post(&self, uri: &str, message: &[u8]) -> Result<String> {
        self.request(
            Method::Post,
            uri,
            Some(message)
        )
    }

    fn patch(&self, uri: &str, message: &[u8]) -> Result<String> {
        self.request(
            Method::Patch,
            uri,
            Some(message)
        )
    }

    fn put(&self, uri: &str, message: &[u8]) -> Result<String> {
        self.request(
            Method::Put,
            uri,
            Some(message)
        )
    }
}
