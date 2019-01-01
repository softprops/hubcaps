extern crate pretty_env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio;

use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::issues::{IssueListOptions, State};
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
            rt.block_on(
                github
                    .repo("matthiasbeyer", "imag")
                    .issues()
                    .iter(
                        &IssueListOptions::builder()
                            .per_page(100)
                            .state(State::All)
                            .build(),
                    )
                    .for_each(move |issue| {
                        println!("{} ({})", issue.title, issue.state);
                        Ok(())
                    }),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
