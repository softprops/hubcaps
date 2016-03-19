# 0.2.0

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

# 0.1.1

* DeploymentStatusOptions now have an optional field for `target_url` and `description`

# 0.1.0

* initial release
