extern crate pretty_env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio;

use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::search::SearchIssuesOptions;
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
            println!("issue search results");
            // https://developer.github.com/v3/search/#parameters-3
            rt.block_on(
                github
                    .search()
                    .issues()
                    .iter(
                        "user:softprops",
                        &SearchIssuesOptions::builder().per_page(100).build(),
                    )
                    .for_each(|issue| {
                        println!("{}", issue.title);
                        Ok(())
                    }),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
