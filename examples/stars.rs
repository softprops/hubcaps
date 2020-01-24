use std::env;

use futures::try_join;

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
            let stars = github.activity().stars();
            let result = try_join!(
                stars.star("softprops", "hubcaps"),
                stars.is_starred("softprops", "hubcaps")
            );
            match result {
                Ok((_, starred)) => println!("starred? {:?}", starred),
                Err(err) => println!("err {}", err),
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
