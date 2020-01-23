use std::env;
use std::str;

use futures::future;
use futures::TryStreamExt;

use hubcaps::content::DirectoryItem;
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
                .await
                .try_for_each(|item| {
                    println!("  {}", item.path);
                    future::ok(())
                })
                .await?;

            println!("Root directory:");
            for item in repo
                .content()
                .root()
                .await
                .try_collect::<Vec<DirectoryItem>>()
                .await?
            {
                println!("  {}", item.path)
            }

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
