use std::env;

use futures::TryStreamExt;

use hubcaps::{Credentials, Github, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;
            let mut iter = github
                .user_repos("softprops")
                .iter(&Default::default())
                .await;
            while let Some(repo) = iter.try_next().await? {
                println!("{}", repo.name);
                let langs = repo.languages(github.clone()).await?;
                for (language, bytes_of_code) in langs {
                    println!("{}: {} bytes", language, bytes_of_code)
                }
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
