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
