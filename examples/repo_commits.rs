use std::env;

use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    let mut rt = Runtime::new()?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        env::var("GITHUB_TOKEN")
            .ok()
            .map(|token| Credentials::Token(token)),
    )?;

    let first_commit = rt.block_on(
        github
            .repo("softprops", "hubcaps")
            .commits()
            .get("1758957ddab20ba17a1fa501f31932d1a9d96f78"),
    )?;
    println!("Check out the first commit: {:#?}", first_commit);

    println!("Here are some more recent commits:");
    let commits = rt.block_on(github.repo("softprops", "hubcaps").commits().list())?;
    for commit in commits {
        println!(" - {}", commit.author.login);
    }
    println!("Thank you for your help!");
    Ok(())
}
