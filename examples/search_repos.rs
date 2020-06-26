use futures::prelude::*;
use hubcaps::search::SearchReposOptions;
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
    println!("repo search results");
    // https://developer.github.com/v3/search/#parameters
    github
        .search()
        .repos()
        .iter(
            "user:softprops hubcaps",
            &SearchReposOptions::builder().per_page(100).build(),
        )
        .try_for_each(|repo| async move {
            println!("{}", repo.full_name);
            Ok(())
        })
        .await?;
    Ok(())
}
