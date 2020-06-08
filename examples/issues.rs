use futures::prelude::*;
use hubcaps::issues::{IssueListOptions, State};
use hubcaps::{Credentials, Github, Result};
use std::env;

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
                .try_for_each(move |issue| async move {
                    println!("{} ({})", issue.title, issue.state);
                    Ok(())
                })
                .await?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
