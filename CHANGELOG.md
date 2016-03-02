# 0.2.0

* port serialization from `rustc-serialize` to `serde`!
* renamed `hubcaps::statuses::State` to `hubcaps::StatusState`
* added `payload` field to `hubcaps::Deployment` represented as a `serde_json::Value`

# 0.1.1

* DeploymentStatusOptions now have an optional field for `target_url` and `description`

# 0.1.0

* initial release
