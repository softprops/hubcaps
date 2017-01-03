//! Hooks interface
extern crate serde_json;

use self::super::{Github, Result};
use rep::{Hook, HookCreateOptions};
use std::fmt;

/// Content-Type web hooks will recieve
/// deliveries in
#[derive(Debug, PartialEq)]
pub enum WebHookContentType {
    /// application/json
    Json,
    /// application/x-form-url-encoded
    Form,
}

impl Default for WebHookContentType {
    fn default() -> WebHookContentType {
        WebHookContentType::Form
    }
}

impl fmt::Display for WebHookContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   WebHookContentType::Form => "form",
                   WebHookContentType::Json => "json",
               })
    }
}

/// Interface for mangaing repository hooks
pub struct Hooks<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Hooks<'a> {
    /// Create a new deployments instance
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Hooks<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Hooks {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// lists hook associated with a repoistory
    pub fn list(&self) -> Result<Vec<Hook>> {
        self.github.get(&format!("/repos/{}/{}/hooks", self.owner, self.repo))
    }

    /// creates a new repository hook
    /// Repository service hooks (like email or Campfire) can have at most one configured at a time.
    /// Creating hooks for a service that already has one configured will update the existing hook.
    /// see [github docs](https://developer.github.com/v3/repos/hooks/)
    /// for more information
    pub fn create(&self, options: &HookCreateOptions) -> Result<Hook> {
        let data = try!(serde_json::to_string(&options));
        self.github.post::<Hook>(&format!("/repos/{}/{}/hooks", self.owner, self.repo),
                                 data.as_bytes())
    }

    /// deletes a repoistory hook by id
    pub fn delete(&self, id: u64) -> Result<()> {
        self.github.delete(&format!("/repos/{}/{}/hooks/{}", self.owner, self.repo, id))
    }
}

#[cfg(test)]
mod tests {
    use super::WebHookContentType;

    #[test]
    fn webhook_content_type_display() {
        for (ct, expect) in vec![(WebHookContentType::Form, "form"),
                                 (WebHookContentType::Json, "json")] {
            assert_eq!(ct.to_string(), expect)
        }
    }

    #[test]
    fn webhook_content_type_default() {
        let default: WebHookContentType = Default::default();
        assert_eq!(default, WebHookContentType::Form)
    }
}
