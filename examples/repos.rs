use futures::prelude::*;
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let github = &Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )?;
    github
        .user_repos("softprops")
        .iter(&Default::default())
        .try_for_each(move |repo| async move {
            println!("{}", repo.name);
            let f = repo.languages(github.clone()).map_ok(|langs| {
                for (language, bytes_of_code) in langs {
                    println!("{}: {} bytes", language, bytes_of_code)
                }
            });
            tokio::spawn(f.map(|_| ()));
            Ok(())
        })
        .await?;
    Ok(())
}
