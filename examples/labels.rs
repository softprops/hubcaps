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
                .await
                .try_for_each(|label| {
                    println!("{}", label.name);
                    future::ok(())
                })
                .await?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
