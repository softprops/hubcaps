# 0.2.0

* port serialization from `rustc-serialize` to `serde`!
* as a result of the serde port, `Error::{Decoding, Encoding}` were removed and replaced with `Error::Serialize`
* renamed `hubcaps::statuses::State` to `hubcaps::StatusState`
* added `payload` field to `hubcaps::Deployment` represented as a `serde_json::Value`
* added `content_type` field to `hubcaps::GistFile` represented as `String`
* added `truncated` field to `hubcaps::Gist` represented as an `bool` and updated `truncated` field of `hubcaps::GistFile` to be `Option<bool>` (this field is omitted in gist listing responses)

# 0.1.1

* DeploymentStatusOptions now have an optional field for `target_url` and `description`

# 0.1.0

* initial release
