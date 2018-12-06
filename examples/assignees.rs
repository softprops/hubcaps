use std::env;

use env_logger;
use hubcaps::{Credentials, Github, Result};
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );
            let pull = rt.block_on(
                github
                    .repo("softprops", "hubcaps")
                    .pulls()
                    .get(122)
                    .assignees()
                    .add(vec!["softprops"]),
            )?;
            println!("{:#?}", pull);

            let issue = rt.block_on(
                github
                    .repo("softprops", "hubcaps")
                    .issues()
                    .get(125)
                    .assignees()
                    .add(vec!["softprops"]),
            )?;
            println!("{:#?}", issue);
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
