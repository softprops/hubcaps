# hubcaps

[![Build Status](https://travis-ci.org/softprops/hubcaps.svg?branch=master)](https://travis-ci.org/softprops/hubcaps)

> a rust interface for github

## docs

Find them [here](http://softprops.github.io/hubcaps)

## usage


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

Doug Tangren (softprops) 2015
