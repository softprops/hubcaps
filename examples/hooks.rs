extern crate pretty_env_logger;
extern crate hubcaps;
extern crate tokio;

use std::env;

use tokio::runtime::Runtime;

use hubcaps::hooks::{HookCreateOptions, WebHookContentType};
use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );
            let repo = github.repo("softprops", "hubcaps");
            let hook = rt.block_on(
                repo.hooks().create(
                    &HookCreateOptions::web()
                        .url("http://localhost:8080")
                        .content_type(WebHookContentType::Json)
                        .build(),
                ),
            );
            println!("{:#?}", hook);
            let hooks = repo.hooks();
            for hook in rt.block_on(hooks.list())? {
                println!("{:#?}", hook)
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
