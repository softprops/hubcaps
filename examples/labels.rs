use futures::prelude::*;
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
            // add labels associated with a pull
            println!(
                "{:#?}",
                github
                    .repo("softprops", "hubcaps")
                    .pulls()
                    .get(121)
                    .labels()
                    .add(vec!["enhancement"])
                    .await?
            );
            // stream over all labels defined for a repo
            github
                .repo("rust-lang", "cargo")
                .labels()
                .iter()
                .try_for_each(move |label| async move {
                    println!("{}", label.name);
                    Ok(())
                })
                .await?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
