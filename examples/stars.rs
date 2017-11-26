extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate futures;
extern crate tokio_core;
#[macro_use(quick_main)]
extern crate error_chain;

use std::env;

use futures::Future;
use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Some(Credentials::Token(token)),
                &core.handle(),
            );
            let stars = github.activity().stars();
            let f = stars.star("softprops", "hubcaps").join(stars.starred(
                "softprops",
                "hubcaps",
            ));
            match core.run(f) {
                Ok((_, starred)) => println!("starred? {:?}", starred),
                Err(err) => println!("err {}", err),
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
