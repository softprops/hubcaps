# 0.6.2

* Fix pagination for GitHub Enterprise [#247](https://github.com/softprops/hubcaps/pull/247)
* Fix jsonwebtoken auth support [#264](https://github.com/softprops/hubcaps/pull/264)
* Fix compile error with httpcache cargo feature [#270](https://github.com/softprops/hubcaps/pull/270)
* Add repository contributor statistics api [](#272)(https://github.com/softprops/hubcaps/pull/272)

# 0.6.1

* patch release just to update crates.io readme

# 0.6.0

* BREAKING CHANGE: Migrate from old `futures` crate futures to std library futures making it possible to use `async`/`await` with this library. This puts this library in better compatibility with the current rust async ecosystem. Please see the examples directory for updated examples of how to use these. [#254](https://github.com/softprops/hubcaps/pull/254)
* BREAKING CHANGE: replace `error_chain` derived errors with `std::error::Error` implementing `Error` enum. The motivation is that the error crate ecosystem is a moving target. The `std::error` package is not. This also makes for a smaller crate and smaller surface area. This moves away from errors of the form `Error(ErrorKind::Codec(_), _)` to errors of the form `Error::Codec(_)`
* Add support for Content create and update apis [#253](https://github.com/softprops/hubcaps/pull/253)
* Add description field to label apis [#252](https://github.com/softprops/hubcaps/pull/252)
* Make status fields `created_at`, `updated_at` and `target_url` optional [#250](https://github.com/softprops/hubcaps/pull/250) [#249](https://github.com/softprops/hubcaps/pull/249)
* Mask sensitive information in the `Debug` impl of `Credentials` type [#261](https://github.com/softprops/hubcaps/pull/261)

# 0.5.0

* BREAKING CHANGE: upgrade to hyper 0.12 and replace `tokio-core` with `tokio` [#136](https://github.com/softprops/hubcaps/pull/136)

This simplifies interfaces for constructing instances as it removes the need to pass a borrowed `Handle` around

before

```rust
let mut core = Core::new().expect("failed to initilaize core");
 Github::new(
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    Credentials::Token(token),
    &core.handle(),
);
```

after

```rust
 Github::new(
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    Credentials::Token(token)
 );
```
* add experimental feature for http [etag](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag) caching [#151](https://github.com/softprops/hubcaps/pull/151) [#160](https://github.com/softprops/hubcaps/pull/160)

This allows clients to keep a local cache of response data to avoid the need to download responses when data hasn't changed
This features is currently behind a feature flag until its stabalized.

You can find an example in this repo with

```sh
$ cargo run --no-default-features --features tls,httpcache --example conditional_requests
```

To enable this feature in your application dependencies add the following to you're `Cargo.toml` file

```toml
[dependencies.hubcaps]
version = "0.5.0"
default-features = false
features = ["tls","httpcache"]
```

* add `pull_request` field to issue struct [#156](https://github.com/softprops/hubcaps/pull/156)
* improve contents API [#155](https://github.com/softprops/hubcaps/pull/155)
* implement repository [contributors api](https://developer.github.com/v3/repos/#list-contributors) [#154](https://github.com/softprops/hubcaps/pull/154)
* add release helper methods to get `latest` release and `release_by_tag` [#147](https://github.com/softprops/hubcaps/pull/147)
* add optional [rustls](https://github.com/ctz/rustls) support


# 0.4.10

* added ability to post review comments [#142](https://github.com/softprops/hubcaps/pull/142)
* added interfaces for [notifications apis](https://developer.github.com/v3/activity/notifications/) [#146](https://github.com/softprops/hubcaps/pull/146)

```rust
github.repo("you", "repo")
  .activity()
  .notifications()
  .list(&Default::default())
```

* added interfaces for [traffic apis](https://developer.github.com/v3/repos/traffic/) [#145](https://github.com/softprops/hubcaps/pull/145)

```rust
github.repo("you", "repo")
  .traffic()
  .clones(TimeUnit::Day)
```

* added interfaces for getting the latest release and release by tag [#147](https://github.com/softprops/hubcaps/pull/147)

```rust
github.repo("you", "repo")
 .releases()
 .latest()
```

# 0.4.9

* add the ability to delete a git ref (tag, branch ect)

```rust
github.repo("you", "repo")
  .git()
  .delete_reference("heads/awesome-feature")
```

# 0.4.8

* fixed bug with `hubcaps::search::IssueItem.repo_tuple()`

# 0.4.7

* added assignee manage interfaces to pull request and issues interfaces
* deserialize issue assignees

```rust
github.repo("you", "repo")
  .pulls()
  .get(number)
  .assignees()
  .add(vec!["your-github-login"])
```

* introduced a minor ergonomic improvement in Github instance creation. Credentials
  are now provided as `Into<Option<Credentials>>` meaning you no longer have to wrap
  credentials with `Some(...)` when providing credentials

before

```rust
let github = Github::new(
  "my-cool-user-agent/0.1.0",
  Some(Credentials::Token("personal-access-token")),
  &core.handle()
);
```

after

```rust
let github = Github::new(
  "my-cool-user-agent/0.1.0",
  Credentials::Token("personal-access-token"),
  &core.handle()
);
```

# 0.4.6

* add support for pull request label deserialization and pull request issue interaction

```rust
github.repo("you", "repo")
  .pulls()
  .get(number)
  .get()
  .inspect(|&pull| println!("{:#?}",pull.labels))

...

github.repo("you", "repo")
  .pulls()
  .get(number)
  .labels()
  .add(vec!["enhancement"])
```

# 0.4.5

* add support for iterating over a stream of repo issues `github.repo(.., ..).issues().iter(opts)`
* support anonymous gist owners [#111](https://github.com/softprops/hubcaps/pull/111)

# 0.4.4

* fix issue with stream pagination [#108](https://github.com/softprops/hubcaps/pull/108)
* implement stream iter for repo labels [#110](https://github.com/softprops/hubcaps/pull/110)
* issue body is now an `Option<String>` type [#107](https://github.com/softprops/hubcaps/pull/107)
* upgrade log dependency `0.3` => `0.4`

# 0.4.3

* fixed url bug with language looking for repositories
* fixed url bug with iter based pagination
* introduce new ErrorKind::RateLimit error for better rate limit detection

# 0.4.2

* add transparent handling of 307 temporary redirect

# 0.4.1

* add transparent handling of 301 permanent moves for repo renames

# 0.4.0

* upgrade to async hyper (0.11)
* begin [stars](https://developer.github.com/v3/activity/starring) interface

## breaking changes

Hyper 0.11's switch to async APIs had a major impact to the API design choices
in this release. The following are the major notable changes

* interfaces that previously returned `hubcaps::Result` types now return `hubcaps::Future` types. The semantics are the same, the difference is that
these map to async computed values. To learn more about Futures and
Future combinators see [this documentation](http://alexcrichton.com/futures-rs/futures/future/index.html)
* `hubcaps::Client`'s associated methods for creating new interfaces got a facelift. The `hyper::Client` previously required for constructor methods is provided by default ( customization is still supported ) with a default tls
connector. A `tokio_core::reactor::Handle` reference is required in order to
construct this client. The motivation is that its the application responsibility
to manage `Core` resources.
* `iter` methods previously returned `Iter` types which provided a way to iterate
over elements of paginated collections. The analog to iterators in the async world `hubcaps::Stream` types which are akin to an iterator in which values are
computed asynchronously. To learn more about Streams and Stream combinators see
[this documentation](http://alexcrichton.com/futures-rs/futures/stream/index.html)
* Credentials are now provided as an Option type removing the need for Credential::None

# 0.3.16

* added users api interfaces [@dpc](https://github.com/softprops/hubcaps/pull/90)

# 0.3.15

* org team description is now an Option type
# 0.3.14

* fixed response parsing for adding branch protection

# 0.3.13

* updated branches interface to reflect [branch API changes](https://developer.github.com/changes/2017-09-06-protected-branches-preview-end/)
* added `SearchIssuesOptions.per_page(n)` interface for limiting search results

# 0.3.12

* fixed issue with persistence of repo term permission

# 0.3.11

* fixed PUT vs PATCH issue with repo team adds

# 0.3.10

* add ability to add team to repository

# 0.3.9

* add support for fetching a single repo by name

# 0.3.8

* add support for org repo creation

# 0.3.7

* add support for updating a repository branch's protection

# 0.3.6

* added `per_page` to various repo list builder interfaces
* fixed org list builder's type filter to use org repo type

# 0.3.5

* hubcaps::git::GitFile's now have an optional url because commits types don't
  have urls.

# 0.3.4

* added git tree and blob fetching interfaces

# 0.3.3

* added org repos interface

# 0.3.2

* use error_chain to generate error types
* add support for posting issue comments [#71](https://github.com/softprops/hubcaps/pull/71)
* add support for repo teams
* add team permissions
* add iter support to branches, repos, pulls, and teams

# 0.3.1

* fix order of Iter traversal

# 0.3.0

* added support for repo hooks
* `Github::new` now takes an owned reference to a hyper::Client. this makes it possible
  to pass a github instance into a threaded context.
* upgrade to serde 0.9 (and now unneeded build.rs machinery)
* sizable code restructure to support scalability in future growth. move foo.rs modules to foo/mod.rs files. moved respective rep.rs reps into mods
* the effect of the above is that everything may no longer be accessible via the top level `hubcaps` module. For instance, in the past you would be able to to access `hubcaps::Pull` directly, now you would access it via is api category `hubcaps::pulls::Pull`.
* update hyper to 0.10. the implications are that you now need to bring your own tls-configured hyper client

# 0.2.8

* expose more pub fields on pull commits

# 0.2.7

* added support for listing pull commits
* added support for returning an iterator over all pull commits

# 0.2.6

* added support for listing issue/pull comments
* added support for listing review comments

# 0.2.5

* added support for search issues api
* add partial support for new Iter type which serves as an transparent iterator over pages of results

# 0.2.4

Improved coverage of pull request api

* Pull.body is now represented as an `Option<String>`
* Pull.assignees is now deserialized
* added `pull.files()` which returns a `Vec<FileDiff>`

# 0.2.3

* added support for repo creation [#38](https://github.com/softprops/hubcaps/pull/38)
* upgrade syntex build dependency to 0.35

# 0.2.2

* upgrade to [hyper 0.8](https://github.com/hyperium/hyper/blob/master/CHANGELOG.md#v080-2016-03-14)
* upgrade syntex build dependency to 0.33

# 0.2.1 (2016-04-09)

* Added support for listing organization repositories [via @carols10cents](https://github.com/softprops/hubcaps/pull/29)
* Fixed deserialization issue related to error response in release api calls [issue #31](https://github.com/softprops/hubcaps/issues/31)

# 0.2.0

Many changes were made to transition into using serde as a serialization backend and to focus on making interfaces more consistent across the board. A more flexible interface for authenticating requests was added as well as a new interface for requesting organization repository listings. Relevant itemized changes are listed below.

* port serialization from `rustc-serialize` to `serde`!
* as a result of the serde port, `Error::{Decoding, Encoding}` which were wrappers around rustc-serialize error types, were removed and replaced with a unified `Error::Codec` which wraps serde's error type
* renamed `hubcaps::statuses::State` to `hubcaps::StatusState`
* added `payload` field to `hubcaps::Deployment` represented as a `serde_json::Value`
* added `content_type` field to `hubcaps::GistFile` represented as `String`
* added `truncated` field to `hubcaps::Gist` represented as an `bool` and updated `truncated` field of `hubcaps::GistFile` to be `Option<bool>` (this field is omitted in gist listing responses)
* introduces `hubcaps::Credentials` as the means of authenticating with the Github api. A `Credentials` value is needed to instantiate a `Github` instance. This is a breaking change from the previous `Option<String>` token api, with a more flexible set options. `hubcaps::Credentials::{None, Token, Client}`. `hubcaps::Credentials` implements `Default` returning `hubcaps::Credentials::None`
* `hubcaps::Error` enum now implements `std::error::Error`
* pull request and issue listing fn's now both take options structs. This is a breaking change.
* repo listing fn's now take option structs. This is a breaking change.
* gist listing fn's now take option structs. This is a breaking change.
* added support for fetching organization repository listings [via @carols10cents](https://github.com/softprops/hubcaps/pull/28)

# 0.1.1

* DeploymentStatusOptions now have an optional field for `target_url` and `description`

# 0.1.0

* initial release
