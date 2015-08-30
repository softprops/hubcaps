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
use hyper::header::{Authorization, UserAgent};
use repository::Repository;
use std::fmt;
use std::io::{Read, Result};

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

  fn get(&self, uri: &str) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    let builder = self.client.get(
      &url
    ).header(
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
    let mut res = authenticated.send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
  }

  fn delete(&self, uri: &str) -> Result<()> {
    let url = format!("{}{}", self.host, uri);
    let builder = self.client.delete(
      &url
    ).header(
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
    let mut res = authenticated.send().unwrap();
    Ok(())
  }

  fn post(&self, uri: &str, message: &[u8]) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    let builder = self.client.post(
      &url
    ).header(
      UserAgent(self.agent.to_owned())
    );
    let authenticated = match self.token {
      Some(token) =>
        builder.header(
          Authorization(format!("token {}", token))
        ),
      _ => builder
    };
    let mut res = authenticated.body(message).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
  }

  fn put(&self, uri: &str, message: &[u8]) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    let builder = self.client.put(
      &url
    ).header(
      UserAgent(self.agent.to_owned())
    );
    let authenticated = match self.token {
      Some(token) =>
        builder.header(
          Authorization(format!("token {}", token))
        ),
      _ => builder
    };
    let mut res = authenticated.body(message).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
  }
}
