use std::env;

use futures::future;
use futures::TryStreamExt;

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
            let repo = github.repo("softprops", "hubcat");
            let pulls = repo.pulls();
            pulls
                .iter(&Default::default())
                .await
                .try_for_each(|pull| {
                    println!("{:#?}", pull);
                    future::ok(())
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
                .await
                .try_for_each(|c| {
                    println!("{:#?}", c);
                    future::ok(())
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
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
