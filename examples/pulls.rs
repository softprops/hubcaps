use futures::prelude::*;
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
    let repo = github.repo("softprops", "hubcat");
    let pulls = repo.pulls();
    pulls
        .iter(&Default::default())
        .try_for_each(|pull| async move {
            println!("{:#?}", pull);
            Ok(())
        })
        .await?;

    println!("comments");
    for c in github
        .repo("softprops", "hubcaps")
        .pulls()
        .get(28)
        .comments()
        .list(&Default::default())
        .await?
    {
        println!("{:#?}", c);
    }

    println!("commits");
    github
        .repo("softprops", "hubcaps")
        .pulls()
        .get(28)
        .commits()
        .iter()
        .try_for_each(|c| async move {
            println!("{:#?}", c);
            Ok(())
        })
        .await?;

    println!("review requests");
    println!(
        "{:#?}",
        github
            .repo("softprops", "hubcaps")
            .pulls()
            .get(190)
            .review_requests()
            .get()
            .await?
    );
    Ok(())
}
