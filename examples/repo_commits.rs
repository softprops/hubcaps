use std::env;

use hubcaps::{Credentials, Github, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        env::var("GITHUB_TOKEN")
            .ok()
            .map(|token| Credentials::Token(token)),
    )?;

    let first_commit = github
        .repo("softprops", "hubcaps")
        .commits()
        .get("1758957ddab20ba17a1fa501f31932d1a9d96f78")
        .await?;
    println!("Check out the first commit: {:#?}", first_commit);

    println!("Here are some more recent commits:");
    let commits = github.repo("softprops", "hubcaps").commits().list().await?;
    for commit in commits {
        println!(" - {}", commit.author.login);
    }
    println!("Thank you for your help!");
    Ok(())
}
