//! Hooks interface
//! See the [github docs](https://developer.github.com/v3/repos/hooks/) for more information

extern crate serde_json;

use self::super::{Github, Result};
use std::fmt;
use std::collections::BTreeMap;

/// Content-Type web hooks will recieve
/// deliveries in
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebHookContentType {
    /// application/json
    #[serde(rename = "json")]
    Json,
    /// application/x-form-url-encoded
    #[serde(rename = "form")]
    Form,
}

impl Default for WebHookContentType {
    fn default() -> WebHookContentType {
        WebHookContentType::Form
    }
}

impl fmt::Display for WebHookContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                WebHookContentType::Form => "form",
                WebHookContentType::Json => "json",
            }
        )
    }
}

/// Interface for mangaing repository hooks
pub struct Hooks<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Hooks<'a> {
    #[doc(hidden)]
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Hooks<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Hooks {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// lists hook associated with a repoistory
    pub fn list(&self) -> Result<Vec<Hook>> {
        self.github.get(&format!(
            "/repos/{}/{}/hooks",
            self.owner,
            self.repo
        ))
    }

    /// creates a new repository hook
    /// Repository service hooks (like email or Campfire) can have at most one configured at a time.
    /// Creating hooks for a service that already has one configured will update the existing hook.
    /// see [github docs](https://developer.github.com/v3/repos/hooks/)
    /// for more information
    pub fn create(&self, options: &HookCreateOptions) -> Result<Hook> {
        let data = serde_json::to_string(&options)?;
        self.github.post::<Hook>(
            &format!("/repos/{}/{}/hooks", self.owner, self.repo),
            data.as_bytes(),
        )
    }

    /// edits an existing repository hook
    pub fn edit(&self, id: u64, options: &HookEditOptions) -> Result<Hook> {
        let data = serde_json::to_string(&options)?;
        self.github.patch::<Hook>(
            &format!("/repos/{}/{}/hooks/{}", self.owner, self.repo, id),
            data.as_bytes(),
        )
    }

    /// deletes a repoistory hook by id
    pub fn delete(&self, id: u64) -> Result<()> {
        self.github.delete(&format!(
            "/repos/{}/{}/hooks/{}",
            self.owner,
            self.repo,
            id
        ))
    }
}

// representations

/// options for creating a repository hook
/// see [this](https://developer.github.com/v3/repos/hooks/#create-a-hook)
/// for githubs official documentation
#[derive(Debug, Serialize)]
pub struct HookCreateOptions {
    name: String,
    config: BTreeMap<String, ::serde_json::Value>,
    events: Vec<String>,
    active: bool,
}

impl HookCreateOptions {
    /// creates a new builder instance with a hook name
    /// are should be taken with respect to the hook name as you can only
    /// use "web" or the a valid service name listed [here](https://api.github.com/hooks)
    pub fn builder<N>(name: N) -> HookCreateOptionsBuilder
    where
        N: Into<String>,
    {
        HookCreateOptionsBuilder::new(name)
    }

    /// use this for creating a builder for webhook options
    pub fn web() -> HookCreateOptionsBuilder {
        Self::builder("web")
    }
}

#[derive(Default)]
pub struct HookCreateOptionsBuilder {
    name: String,
    config: BTreeMap<String, ::serde_json::Value>,
    events: Vec<String>,
    active: bool,
}

impl HookCreateOptionsBuilder {
    #[doc(hidden)]
    pub fn new<N>(name: N) -> HookCreateOptionsBuilder
    where
        N: Into<String>,
    {
        HookCreateOptionsBuilder {
            name: name.into(),
            active: true,
            ..Default::default()
        }
    }


    pub fn active(&mut self, active: bool) -> &mut Self {
        self.active = active;
        self
    }

    /// a list of github events this hook should receive deliveries for
    /// the default is "push". for a full list, see
    /// the [Github api docs](https://developer.github.com/webhooks/#events)
    pub fn events<E>(&mut self, events: Vec<E>) -> &mut Self
    where
        E: Into<String>,
    {
        self.events = events.into_iter().map(|e| e.into()).collect::<Vec<_>>();
        self
    }

    /// web hooks must have an associated url
    pub fn url<U>(&mut self, url: U) -> &mut Self
    where
        U: Into<String>,
    {
        self.config_entry("url".to_owned(), ::serde_json::Value::String(url.into()))
    }

    /// web hooks can optionally specify a content_type of "form" or "json"
    /// which indicates the type of payload they will expect to receive
    pub fn content_type(&mut self, content_type: WebHookContentType) -> &mut Self {
        self.config_str_entry("content_type", content_type.to_string());
        self
    }

    /// web hooks can optionally provide a secret used to sign deliveries
    /// to identify that their source was indeed github
    pub fn secret<S>(&mut self, sec: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.config_str_entry("secret", sec);
        self
    }

    pub fn config_str_entry<K, V>(&mut self, k: K, v: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.config_entry(k.into(), ::serde_json::Value::String(v.into()));
        self
    }

    pub fn config_entry<N>(&mut self, name: N, value: ::serde_json::Value) -> &mut Self
    where
        N: Into<String>,
    {
        self.config.insert(name.into(), value);
        self
    }

    pub fn build(&self) -> HookCreateOptions {
        HookCreateOptions {
            name: self.name.clone(),
            config: self.config.clone(),
            events: self.events.clone(),
            active: self.active,
        }
    }
}


/// options for editing a repository hook
/// see [this](https://developer.github.com/v3/repos/hooks/#edit-a-hook)
/// for githubs official documentation
#[derive(Debug, Serialize)]
pub struct HookEditOptions {
    config: BTreeMap<String, ::serde_json::Value>,
    events: Vec<String>,
    add_events: Vec<String>,
    remove_events: Vec<String>,
    active: bool,
}

impl HookEditOptions {
    /// creates a new builder instance
    pub fn builder() -> HookEditOptionsBuilder {
        HookEditOptionsBuilder::new()
    }
}

#[derive(Default)]
pub struct HookEditOptionsBuilder {
    config: BTreeMap<String, ::serde_json::Value>,
    events: Vec<String>,
    add_events: Vec<String>,
    remove_events: Vec<String>,
    active: bool,
}

impl HookEditOptionsBuilder {
    pub fn new() -> HookEditOptionsBuilder {
        HookEditOptionsBuilder { ..Default::default() }
    }


    pub fn active(&mut self, active: bool) -> &mut Self {
        self.active = active;
        self
    }

    /// a list of github events this hook should receive deliveries for
    /// the default is "push". for a full list, see
    /// the [Github api docs](https://developer.github.com/webhooks/#events)
    pub fn events<E>(&mut self, events: Vec<E>) -> &mut Self
    where
        E: Into<String>,
    {
        self.events = events.into_iter().map(|e| e.into()).collect::<Vec<_>>();
        self
    }

    /// web hooks must have an associated url
    pub fn url<U>(&mut self, url: U) -> &mut Self
    where
        U: Into<String>,
    {
        self.config_entry("url".to_owned(), ::serde_json::Value::String(url.into()))
    }

    /// web hooks can optionally specify a content_type of "form" or "json"
    /// which indicates the type of payload they will expect to receive
    pub fn content_type(&mut self, content_type: WebHookContentType) -> &mut Self {
        self.config_str_entry("content_type", content_type.to_string());
        self
    }

    /// web hooks can optionally provide a secret used to sign deliveries
    /// to identify that their source was indeed github
    pub fn secret<S>(&mut self, sec: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.config_str_entry("secret", sec);
        self
    }

    pub fn config_str_entry<K, V>(&mut self, k: K, v: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.config_entry(k.into(), ::serde_json::Value::String(v.into()));
        self
    }

    pub fn config_entry<N>(&mut self, name: N, value: ::serde_json::Value) -> &mut Self
    where
        N: Into<String>,
    {
        self.config.insert(name.into(), value);
        self
    }

    pub fn build(&self) -> HookEditOptions {
        HookEditOptions {
            config: self.config.clone(),
            events: self.events.clone(),
            add_events: self.add_events.clone(),
            remove_events: self.remove_events.clone(),
            active: self.active,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Hook {
    pub id: u64,
    pub url: String,
    pub test_url: String,
    pub ping_url: String,
    pub name: String,
    pub events: Vec<String>,
    pub config: ::serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
    pub active: bool,
}

impl Hook {
    pub fn config_value(&self, name: &str) -> Option<&::serde_json::Value> {
        self.config.pointer(&format!("/{}", name))
    }

    pub fn config_string(&self, name: &str) -> Option<String> {
        self.config_value(name).and_then(|value| match *value {
            ::serde_json::Value::String(ref val) => Some(val.clone()),
            _ => None,
        })
    }

    pub fn url(&self) -> Option<String> {
        self.config_string("url")
    }

    pub fn content_type(&self) -> Option<String> {
        self.config_string("content_type")
    }
}

#[cfg(test)]
mod tests {
    use super::WebHookContentType;

    #[test]
    fn webhook_content_type_display() {
        for (ct, expect) in vec![
            (WebHookContentType::Form, "form"),
            (WebHookContentType::Json, "json"),
        ]
        {
            assert_eq!(ct.to_string(), expect)
        }
    }

    #[test]
    fn webhook_content_type_default() {
        let default: WebHookContentType = Default::default();
        assert_eq!(default, WebHookContentType::Form)
    }
}
