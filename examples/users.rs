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
            match github.users().authenticated().await {
                Ok(me) => println!("{:#?}", me),
                Err(err) => println!("err {:#?}", err),
            }

            match github
                .users()
                .get(
                    env::var("GH_USERNAME")
                        .ok()
                        .unwrap_or_else(|| "bors".into()),
                )
                .await
            {
                Ok(user) => println!("{:#?}", user),
                Err(err) => println!("err {:#?}", err),
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
