

<h1 align="center">
  hubcaps
</h1>

<p align="center">
   a Rust interface for GitHub
</p>

<div align="center">
  <a alt="GitHub Actions" href="https://github.com/softprops/hubcaps/actions">
    <img src="https://github.com/softprops/hubcaps/workflows/Main/badge.svg"/>
  </a>
  <a alt="crates.io" href="https://crates.io/crates/hubcaps">
    <img src="https://img.shields.io/crates/v/hubcaps.svg?logo=rust"/>
  </a>
  <a alt="docs.rs" href="http://docs.rs/hubcaps">
    <img src="https://docs.rs/hubcaps/badge.svg"/>
  </a>
  <a alt="latest docs" href="https://softprops.github.io/hubcaps">
   <img src="https://img.shields.io/badge/docs-latest-green.svg"/>
  </a>
  <a alt="license" href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-brightgreen.svg"/>
  </a>
</div>

<br />

## installation

Add the following to your `Cargo.toml` file

```toml
[dependencies]
hubcaps = "0.5"
```

## usage

Basic usage requires a user agent string and
optionally a flavor of `hubcaps::Credentials` for making requests as a particular
GitHub user.

For user authenticated requests you'll typically want to use
`hubcaps::Credentials::Token` with a
[personal access token](https://github.com/settings/tokens).

```rust
use hubcaps::{Credentials, Github};

fn main() {
  let github = Github::new(
    "my-cool-user-agent/0.1.0",
    Credentials::Token("personal-access-token"),
  );
}
```

GitHub instances define methods for accessing api services that map closely to
their url structure.

As a convention, api methods that expect arguments are represented as functions
that accept a struct representing those arguments with an optional builder
interface for convenience of construction.

See [examples directory](examples/) for some getting started examples

### repositories

Typically the reference point of most GitHub services is a repository

```rust
let repo = github.repo("user", "repo");
```

With a repo instance on hand, you can access a number of sub services,
like `labels`, `deployments`, `pulls`, `issues`, `releases`, and many more.
Each of this are named functions exported from the repo interface.

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
labels.create(
    &LabelOptions::new(
      "rustic", "ccc"
    )
  )
```

### deployments

Deployments is a service for orchestrating deployments of applications sourced from GitHub repositories

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


### hooks

Hooks is a service for managing repository hooks

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

Doug Tangren (softprops) 2015-2020
