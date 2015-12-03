
//! Hubcaps provides a set of building blocks for interacting with the Github API

extern crate hyper;
extern crate rustc_serialize;
extern crate url;

use rustc_serialize::{json, Decodable};
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
pub mod pulls;

pub use rep::*;
pub use errors::Error;
use gists::{Gists, UserGists};
use hyper::Client;
use hyper::method::Method;
use hyper::header::{Authorization, ContentLength, UserAgent};
use hyper::status::StatusCode;
use repositories::{Repository, Repositories, UserRepositories};
use std::fmt;
use std::io::Read;

const DEFAULT_HOST: &'static str = "https://api.github.com";

/// alias for Result that infers hubcaps::Error as Err
pub type Result<T> = std::result::Result<T, Error>;

/// enum representation of github pull and issue state
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
    host: String,
    agent: String,
    client: &'a Client,
    token: Option<String>
}

impl<'a> Github<'a> {
    /// Create a new Github instance
    pub fn new<A,T>(
        agent: A,
        client: &'a Client,
        token: Option<T>
    ) -> Github<'a> where A : Into<String>, T: Into<String> {
        Github::host(DEFAULT_HOST, agent, client, token)
    }

    /// Create a new Github instance hosted at a custom location.
    /// Useful for github enterprise installations ( yourdomain.com/api/v3/ )
    pub fn host<H,A,T>(
        host: H,
        agent: A,
        client: &'a Client,
        token: Option<T>
     ) -> Github<'a> where H: Into<String>, A: Into<String>, T: Into<String>{
        Github {
            host: host.into(),
            agent: agent.into(),
            client: client,
            token: token.map(|t| t.into())
        }
    }

    /// Return a reference to a Github reposistory
    pub fn repo<O,R>(&self, owner: O, repo: R) -> Repository where O: Into<String>, R: Into<String> {
        Repository::new(self, owner, repo)
    }

    /// Return a reference to the collection of repositories owned by an
    /// associated with an owner
    pub fn user_repos<S>(&self, owner: S) -> UserRepositories where S: Into<String>{
        UserRepositories::new(self, owner)
    }

    /// Return a reference to  the collection of repositores owned by the user
    /// associated with the current authentication credentials
    pub fn repos(&self) -> Repositories {
        Repositories::new(self)
    }

    /// Return a reference to an interface that provides access to a user's gists
    pub fn user_gists<O>(&self, owner: O) -> UserGists where O: Into<String> {
        UserGists::new(self, owner)
    }

    /// Return a reference to an interface that provides access to the
    /// gists belonging to the owner of the token used to configure this client
    pub fn gists(&self) -> Gists {
        Gists::new(self)
    }

    fn request<D>(
        &self, method: Method, uri: &str, body: Option<&'a [u8]>) -> Result<D> where D : Decodable {
        let url = format!("{}{}", self.host, uri);
        let builder = self.client.request(method, &url).header(
            UserAgent(self.agent.to_owned())
        );
        let authenticated = match self.token {
            Some(ref token) =>
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
                Error::Fault {
                    code: res.status,
                    error: try!(json::decode::<ClientError>(&body))
                }
            ),
            _ =>
                Ok(try!(json::decode::<D>(&body)))
        }
    }

    fn get<D>(&self, uri: &str) -> Result<D> where D: Decodable {
        self.request(
            Method::Get,
            uri,
            None
        )
    }

    fn delete(&self, uri: &str) -> Result<()> {
        match self.request::<()>(
            Method::Delete,
            uri,
            None
        ) {
            Err(Error::Decoding(_)) => Ok(()),
            otherwise => otherwise
        }
    }

    fn post<D>(&self, uri: &str, message: &[u8]) -> Result<D> where D: Decodable {
        self.request(
            Method::Post,
            uri,
            Some(message)
        )
    }

    fn patch<D>(&self, uri: &str, message: &[u8]) -> Result<D> where D: Decodable {
        self.request(
            Method::Patch,
            uri,
            Some(message)
        )
    }

    fn put<D>(&self, uri: &str, message: &[u8]) -> Result<D> where D: Decodable {
        self.request(
            Method::Put,
            uri,
            Some(message)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
