# 0.2.5 (unreleased)

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
* added support for fetching organization repoistory listings [via @carols10cents](https://github.com/softprops/hubcaps/pull/28)

# 0.1.1

* DeploymentStatusOptions now have an optional field for `target_url` and `description`

# 0.1.0

* initial release
