//! Hubcaps provides a set of building blocks for interacting with the Github API

#[macro_use]
extern crate serializable_enum;
#[macro_use]
extern crate log;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

use serde::de::Deserialize;
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
pub mod organizations;

pub use rep::*;
pub use errors::Error;
use gists::{Gists, UserGists};
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

    /// Return a reference to the collection of repositories owned by and
    /// associated with an organization
    pub fn org_repos<O>(&self, org: O) -> OrganizationRepositories
        where O: Into<String>
    {
        OrganizationRepositories::new(self, org)
    }

    fn authenticate(&self, method: Method, uri: &str) -> RequestBuilder {
        let url = format!("{}{}", self.host, uri);
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

    fn request<D>(&self, method: Method, uri: &str, body: Option<&'a [u8]>) -> Result<D>
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
        debug!("rec response {:#?} {:#?} {}",
               res.status,
               res.headers,
               body);
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
            _ => Ok(try!(serde_json::from_str::<D>(&body))),
        }
    }

    fn get<D>(&self, uri: &str) -> Result<D>
        where D: Deserialize
    {
        self.request(Method::Get, uri, None)
    }

    fn delete(&self, uri: &str) -> Result<()> {
        match self.request::<()>(Method::Delete, uri, None) {
            Err(Error::Codec(_)) => Ok(()),
            otherwise => otherwise,
        }
    }

    fn post<D>(&self, uri: &str, message: &[u8]) -> Result<D>
        where D: Deserialize
    {
        self.request(Method::Post, uri, Some(message))
    }

    fn patch<D>(&self, uri: &str, message: &[u8]) -> Result<D>
        where D: Deserialize
    {
        self.request(Method::Patch, uri, Some(message))
    }

    fn put<D>(&self, uri: &str, message: &[u8]) -> Result<D>
        where D: Deserialize
    {
        self.request(Method::Put, uri, Some(message))
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

    #[test]
    fn default_credentials() {
        let default: Credentials = Default::default();
        assert_eq!(default, Credentials::None)
    }
}
