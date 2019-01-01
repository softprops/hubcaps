extern crate pretty_env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio;

use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::search::SearchReposOptions;
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
            println!("repo search results");
            // https://developer.github.com/v3/search/#parameters
            rt.block_on(
                github
                    .search()
                    .repos()
                    .iter(
                        "user:softprops hubcaps",
                        &SearchReposOptions::builder().per_page(100).build(),
                    )
                    .for_each(|repo| {
                        println!("{}", repo.full_name);
                        Ok(())
                    }),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
