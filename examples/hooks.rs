extern crate env_logger;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::hooks::{HookCreateOptions, WebHookContentType};
use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            let repo = github.repo("softprops", "hubcaps");
            let hook = core.run(
                repo.hooks().create(&HookCreateOptions::web()
                    .url("http://localhost:8080")
                    .content_type(WebHookContentType::Json)
                    .build()),
            );
            println!("{:#?}", hook);
            let hooks = repo.hooks();
            for hook in core.run(hooks.list())? {
                println!("{:#?}", hook)
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
