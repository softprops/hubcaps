use hubcaps::{Credentials, Github, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;
            for gist in github.gists().list(&Default::default()).await? {
                println!("{:#?}", gist)
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
