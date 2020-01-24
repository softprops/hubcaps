use std::env;

use futures::future;
use futures::TryStreamExt;

use hubcaps::issues::{IssueListOptions, State};
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
            github
                .repo("matthiasbeyer", "imag")
                .issues()
                .iter(
                    &IssueListOptions::builder()
                        .per_page(100)
                        .state(State::All)
                        .build(),
                )
                .await
                .try_for_each(|issue| {
                    println!("{} ({})", issue.title, issue.state);
                    future::ok(())
                })
                .await?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
