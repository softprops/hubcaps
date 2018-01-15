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
use hubcaps::comments::CommentOptions;

const USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(USER_AGENT, Some(Credentials::Token(token)), &core.handle());

            let issue = github.repo("softprops", "hubcat").issues().get(1);
            let f = issue.comments().create(&CommentOptions {
                body: format!("Hello, world!\n---\nSent by {}", USER_AGENT),
            });

            match core.run(f) {
                Ok(comment) => println!("{:?}", comment),
                Err(err) => println!("err {}", err),
            }

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
