extern crate pretty_env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio;

use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );
            // add labels associated with a pull
            println!(
                "{:#?}",
                rt.block_on(
                    github
                        .repo("softprops", "hubcaps")
                        .pulls()
                        .get(121)
                        .labels()
                        .add(vec!["enhancement"])
                )?
            );
            // stream over all labels defined for a repo
            rt.block_on(github.repo("rust-lang", "cargo").labels().iter().for_each(
                move |label| {
                    println!("{}", label.name);
                    Ok(())
                },
            ))?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
