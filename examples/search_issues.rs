use std::env;

use futures::future;
use futures::TryStreamExt;

use hubcaps::search::SearchIssuesOptions;
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
            println!("issue search results");
            // https://developer.github.com/v3/search/#parameters-3
            github
                .search()
                .issues()
                .iter(
                    "user:softprops",
                    &SearchIssuesOptions::builder().per_page(100).build(),
                )
                .await
                .try_for_each(|issue| {
                    println!("{}", issue.title);
                    future::ok(())
                })
                .await?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
