use std::env;

use env_logger;
use tokio::runtime::Runtime;

use hubcaps::comments::CommentOptions;
use hubcaps::{Credentials, Github, Result};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn main() -> Result<()> {
    env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(USER_AGENT, Credentials::Token(token));

            let issue = github.repo("softprops", "hubcat").issues().get(1);
            let f = issue.comments().create(&CommentOptions {
                body: format!("Hello, world!\n---\nSent by {}", USER_AGENT),
            });

            match rt.block_on(f) {
                Ok(comment) => println!("{:?}", comment),
                Err(err) => println!("err {}", err),
            }

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
