extern crate env_logger;
extern crate hubcaps;
extern crate tokio;

use std::env;

use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );
            let owner = "softprops";
            let repo = "hubcaps";

            println!("Root directory");
            for file in rt.block_on(github.repo(owner, repo).content().root())? {
                println!("{:#?}", file)
            }

            println!("One file - LICENSE");
            let license = rt.block_on(github.repo(owner, repo).content().file("LICENSE"))?;
            println!("{:#?}", license);

            println!("Directory - examples");
            for example in rt.block_on(github.repo(owner, repo).content().directory("/examples"))? {
                println!("{:#?}", example)
            }

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
