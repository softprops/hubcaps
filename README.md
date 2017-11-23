# hubcaps

[![Build Status](https://travis-ci.org/softprops/hubcaps.svg?branch=master)](https://travis-ci.org/softprops/hubcaps) [![Coverage Status](https://coveralls.io/repos/softprops/hubcaps/badge.svg?branch=master&service=github)](https://coveralls.io/github/softprops/hubcaps?branch=master) [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE) [![crates.io](http://meritbadge.herokuapp.com/hubcaps)](https://crates.io/crates/hubcaps)

> a rust interface for github

[Documentation](http://softprops.github.io/hubcaps)

## /!\ planned API changes

* implement async interface when hyper adopts it

The goal and motivation behind these are not to intentionally make breaking changes,
but rather to adopt evolving community standards

## installation

Add the following to your `Cargo.toml` filter

```toml
[dependencies]
hubcaps = "0.3"
```

## usage

Basic usage requires a user-defined user agent string (because github requires this),
a `hyper::Client` instance, and a flavor of `hubcaps::Credentials` for authorization.
For user authenticated requests you'll typically want to use `hubcaps::Credentials::Token` with a  [personal access token](https://github.com/settings/tokens). For requests that permit anonymous access, you can substitute `hubcaps::Credentials::Token` with `hubcaps::Credentials::None`

> Note: hyper 0.10 no longer includes a tls implementation by default. you will need
>       to provide one to your choosing

```rust
extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;

use tokio_core::reactor::Core;
use hubcaps::{Credentials, Github};

fn main() {
  let mut core = Core::new().expect("reactor fail");
  let github = Github::new(
    "my-cool-user-agent/0.1.0",
    Credentials::Token("personal-access-token"),
    &core.handle()
  );
}
```

Github instances define functions for accessing api services that map closely to their url structure.

As a convention, api methods that expect arguments are represented as functions that accept a struct representing those arguments with an optional builder interface for convenience of construction.


See [examples directory](examples/) for some getting started examples

### repositories

Typically the reference point of most github services is a repository

```rust
let repo = github.repo("user", "repo");
```

With a repo instance on hand, you can access a number of sub services, like `labels`, `deployments`, `pulls`, `issues`, and `releases`. Each of this are named functions exported from the repo interface.

See [examples directory](examples/repos.rs) for examples

### branches

Branches is a service for listing repository branches

```rust
let branches = repo.branches();
```

### labels

Labels is a service for tagging resources like issues and pulls with names which you can later group and filter on.

```rust
use hubcaps::labels::LabelOptions;

let labels = repo.labels();

// create new labels
println!(
  "{:?}", labels.create(
    &LabelOptions::new(
      "rustic", "ccc"
    )
  ).unwrap()
);

// list labels
for l in labels.list().unwrap() {
  println!("{:?}", l)
}

// delete labels
labels.delete("rustic").unwrap();
```

### deployments

Deployments is a service for orchestrating deployments of applications sourced from github repositories

```rust
let deployments = repo.deployments();
```

See [examples directory](examples/deployments.rs) for examples

### pulls

Pulls is a service for issuing code change requests against a repository

```rust
let pulls = repo.pulls();
```

See [examples directory](examples/pulls.rs) for examples

### issues

Issues is a service for tracking bugs for a repository

```rust
let issues = repo.issues();
```

See [examples directory](examples/issues.rs) for examples

### releases

Releases is a service for tracking changes for a stable releases of a versioned library or application

```rust
let releases = repo.releases();
```

### gists

Gists is a service for micro repositories

```rust
let gists = github.gists();
```

See [examples directory](examples/gists.rs) for examples


### gists

Gists is a service for managing repository hooks

```rust
let hooks = repo.hooks();
```

See [examples directory](examples/hooks.rs) for examples

### search

Search provides a raw string query search for indexed data. Currently only search for issues is supported

```rust
let search_issues = github.search().issues();
```

### teams

Teams is a service for listing repository and organization teams

```rust
let teams = repo.teams();
```

See [examples directory](examples/teams.rs) for examples

Doug Tangren (softprops) 2015-2017
