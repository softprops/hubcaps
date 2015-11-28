# hubcaps

[![Build Status](https://travis-ci.org/softprops/hubcaps.svg?branch=master)](https://travis-ci.org/softprops/hubcaps) [![Coverage Status](https://coveralls.io/repos/softprops/hubcaps/badge.svg?branch=master&service=github)](https://coveralls.io/github/softprops/hubcaps?branch=master)

> a rust interface for github

## docs

Find them [here](http://softprops.github.io/hubcaps)

## usage

Basic usage requires a user-defined useragent string, a `hyper::Client` instance and optionally a personal access token.

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

### repositories

Typically the reference point of most github services is a repository

```rust
let repo = github.repo("user", "repo");
```

With a repo on hand, you can access a number of sub services, like `labels`, `deployments`, `pulls`, `issues`, and `releases`.

### labels

Labels is a service for tagging resources like issues and pulls with names which you can later group and filter on.

```rust
use hubcaps::LabelReq;

let labels = repo.labels();

// create new labels
println!(
  "{:?}", labels.create(
    &LabelReq::new(
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

Deployments is a service for orchestation deployments of applications sourced from github repositories

```rust
let deployments = repo.deployments();
```

### pulls

Pulls is a service for issuing change requests against a repository

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

Doug Tangren (softprops) 2015
