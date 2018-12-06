use std::env;

use env_logger;
use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );
            rt.block_on(github.activity().stars().star("rust-lang", "log"))?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
