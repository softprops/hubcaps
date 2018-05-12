extern crate env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use futures::Stream;
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
            println!("org teams");
            core.run(
                github
                    .org("meetup")
                    .teams()
                    .iter()
                    .for_each(|team| Ok(println!("{:#?}", team))),
            )?;
            println!("repo teams");
            core.run(
                github
                    .repo("meetup", "k8s-nginx-dogstats")
                    .teams()
                    .iter()
                    .for_each(|team| Ok(println!("{:#?}", team))),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
