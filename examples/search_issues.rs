use futures::prelude::*;
use hubcaps::search::SearchIssuesOptions;
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
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
        .try_for_each(|issue| async move {
            println!("{}", issue.title);
            Ok(())
        })
        .await?;
    Ok(())
}
