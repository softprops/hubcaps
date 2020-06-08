use std::env;
use std::str;

use futures::prelude::*;

use hubcaps::{Credentials, Github, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;

            let repo = github.repo("softprops", "hubcaps");

            println!("License file:");
            let license = repo.content().file("LICENSE").await?;
            println!("{}", str::from_utf8(&license.content).unwrap());

            println!("Directory contents stream:");
            repo.content()
                .iter("/examples")
                .try_for_each(|item| async move {
                    println!("  {}", item.path);
                    Ok(())
                })
                .await?;

            println!("Root directory:");
            for item in repo.content().root().try_collect::<Vec<_>>().await? {
                println!("  {}", item.path)
            }

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
