extern crate env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::comments::CommentOptions;
use hubcaps::{Credentials, Github, Result};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn main() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(USER_AGENT, Credentials::Token(token), &core.handle());

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
