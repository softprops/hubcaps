# hubcaps

[![Build Status](https://travis-ci.org/softprops/hubcaps.svg?branch=master)](https://travis-ci.org/softprops/hubcaps) [![Coverage Status](https://coveralls.io/repos/softprops/hubcaps/badge.svg?branch=master&service=github)](https://coveralls.io/github/softprops/hubcaps?branch=master) [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

> a rust interface for github

## docs

Find them [here](http://softprops.github.io/hubcaps)

## usage

Basic usage requires a user-defined useragent string, a `hyper::Client` instance and optionally a [personal access token](https://github.com/settings/tokens).

```rust
extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::Github;

fn main() {
  let client = Client::new();
  let github = Github::new(
    "my-cool-user-agent/0.1.0",
    &client,
    Some("personal-access-token")
  );
}
```

Github instances define functions for accessing api services that map closely to their url structure.

As a convention, api methods that expect arguments are represented as functions that accept a struct representing those arguments with an optional builder interface for convenience.

### repositories

Typically the reference point of most github services is a repository

```rust
let repo = github.repo("user", "repo");
```

With a repo instance on hand, you can access a number of sub services, like `labels`, `deployments`, `pulls`, `issues`, and `releases`. Each of this are named functions exported from the repo interface.

### labels

Labels is a service for tagging resources like issues and pulls with names which you can later group and filter on.

```rust
use hubcaps::LabelOptions;

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

Deployments is a service for orchestating deployments of applications sourced from github repositories

```rust
let deployments = repo.deployments();
```

### pulls

Pulls is a service for issuing code change requests against a repository

```rust
let pulls = repo.pulls();
```

### issues

Issues is a service for tracking bugs for a repository

```rust
let issues = repo.issues();
```

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

Doug Tangren (softprops) 2015-2016
