extern crate pretty_env_logger;
extern crate hubcaps;
extern crate tokio;

use std::env;

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
            if let Some(file) = rt
                .block_on(
                    github
                        .repo("softprops", "hubcaps")
                        .git()
                        .tree("master", true),
                )?
                .tree
                .iter()
                .find(|file| file.path == "README.md")
            {
                let blob = rt.block_on(
                    github
                        .repo("softprops", "hubcaps")
                        .git()
                        .blob(file.sha.clone()),
                )?;
                println!("readme {:#?}", blob);
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
