use std::env;

use futures::future;
use futures::TryStreamExt;

use hubcaps::search::SearchReposOptions;
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
            println!("repo search results");
            // https://developer.github.com/v3/search/#parameters
            github
                .search()
                .repos()
                .iter(
                    "user:softprops hubcaps",
                    &SearchReposOptions::builder().per_page(100).build(),
                )
                .await
                .try_for_each(|repo| {
                    println!("{}", repo.full_name);
                    future::ok(())
                })
                .await?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
