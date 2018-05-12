extern crate env_logger;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            for gist in core.run(github.gists().list(&Default::default()))? {
                println!("{:#?}", gist)
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
