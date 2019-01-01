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
            match rt.block_on(github.users().authenticated()) {
                Ok(me) => println!("{:#?}", me),
                Err(err) => println!("err {:#?}", err),
            }

            match rt.block_on(
                github.users().get(
                    env::var("GH_USERNAME")
                        .ok()
                        .unwrap_or_else(|| "bors".into()),
                ),
            ) {
                Ok(user) => println!("{:#?}", user),
                Err(err) => println!("err {:#?}", err),
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
