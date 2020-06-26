use hubcaps::comments::CommentOptions;
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let github = Github::new(USER_AGENT, Credentials::Token(token))?;

    let issue = github.repo("softprops", "hubcat").issues().get(1);
    let f = issue.comments().create(&CommentOptions {
        body: format!("Hello, world!\n---\nSent by {}", USER_AGENT),
    });

    match f.await {
        Ok(comment) => println!("{:?}", comment),
        Err(err) => println!("err {}", err),
    }

    Ok(())
}
