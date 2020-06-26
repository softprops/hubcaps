use hubcaps::traffic::TimeUnit;
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
    let owner = "softprops";
    let repo = "hubcaps";

    println!("Top 10 referrers");
    for referrer in github.repo(owner, repo).traffic().referrers().await? {
        println!("{:#?}", referrer)
    }

    println!("Top 10 paths");
    for path in github.repo(owner, repo).traffic().paths().await? {
        println!("{:#?}", path)
    }

    println!("Views per day");
    let views = github
        .repo(owner, repo)
        .traffic()
        .views(TimeUnit::Day)
        .await?;
    println!("{:#?}", views);

    println!("Clones per day");
    let clones = github
        .repo(owner, repo)
        .traffic()
        .clones(TimeUnit::Day)
        .await?;
    println!("{:#?}", clones);
    Ok(())
}
