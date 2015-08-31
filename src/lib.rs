extern crate hyper;
extern crate rustc_serialize;

pub mod gists;
pub mod deployments;
pub mod issues;
pub mod rep;
pub mod repository;
pub mod pullrequests;

use gists::{Gists, UserGists};
use hyper::Client;
use hyper::client::{IntoUrl, RequestBuilder};
use hyper::method::Method;
use hyper::header::{Authorization, UserAgent};
use repository::Repository;
use std::default::Default;
use std::fmt;
use std::io::{Read, Result};

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

pub struct Github<'a> {
  host: &'static str,
  agent: &'static str,
  client: &'a Client,
  token: Option<&'static str>
}

impl<'a> Github<'a> {
  /// create a new Github instance
  pub fn new(
    agent: &'static str, client: &'a Client, token: Option<&'static str>) -> Github<'a> {
    Github::host("https://api.github.com", agent, client, token)
  }

  /// create a new Github instance hosted and a custom location
  pub fn host(
    host: &'static str, agent: &'static str,
    client: &'a Client, token: Option<&'static str>) -> Github<'a> {
    Github {
      host: host,
      agent: agent,
      client: client,
      token: token
    }
  }

  /// return a reference to a github reposistory
  pub fn repo(&self, owner: &'static str, repo: &'static str) -> Repository {
    Repository::new(self, owner, repo)
  }

  /// return a reference to an interface that provides access to a user's gists
  pub fn user_gists(&self, owner: &'static str) -> UserGists {
    UserGists::new(self, owner)
  }

  /// return a reference to an interface that provides access to the
  /// the gists belonging to the owner of the token used to configure this client
  pub fn gists(&self) -> Gists {
    Gists::new(self)
  }

  fn request<U: IntoUrl>(
    &self, request_builder: RequestBuilder<'a, U>, body: Option<&'a [u8]>) -> Result<String> {
    let builder = request_builder.header(
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
    let mut res = match body {
      Some(ref bod) => authenticated.body(*bod).send().unwrap(),
       _ => authenticated.send().unwrap()
    };
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
  }

  fn get(&self, uri: &str) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    self.request(
      self.client.get(
        &url
      ), None
    )
  }

  fn delete(&self, uri: &str) -> Result<()> {
    let url = format!("{}{}", self.host, uri);
    self.request(
      self.client.delete(
        &url
      ), None
    ).map(|_| ())
  }

  fn post(&self, uri: &str, message: &[u8]) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    self.request(
      self.client.post(
        &url
      ),
      Some(message)
    )
  }

  fn patch(&self, uri: &str, message: &[u8]) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    self.request(
      self.client.request(
        Method::Patch,
        &url
      ),
      Some(message)
    )
  }

  fn put(&self, uri: &str, message: &[u8]) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    self.request(
      self.client.put(
        &url
      ),
      Some(message)
    )
  }
}
