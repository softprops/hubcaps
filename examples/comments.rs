use std::env;

use hubcaps::comments::CommentOptions;
use hubcaps::{Credentials, Github, Result};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(USER_AGENT, Credentials::Token(token))?;

            let issue = github.repo("softprops", "hubcat").issues().get(1);
            let opts = CommentOptions {
                body: format!("Hello, world!\n---\nSent by {}", USER_AGENT),
            };
            match issue.comments().create(&opts).await {
                Ok(comment) => println!("{:?}", comment),
                Err(err) => println!("err {}", err),
            }

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
