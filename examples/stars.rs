use futures::prelude::*;
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )?;
    let stars = github.activity().stars();
    let f = futures::future::try_join(
        stars.star("softprops", "hubcaps"),
        stars.is_starred("softprops", "hubcaps"),
    );
    match f.await {
        Ok((_, starred)) => println!("starred? {:?}", starred),
        Err(err) => println!("err {}", err),
    }

    stars
        .iter("softprops")
        .try_for_each(|s| async move {
            println!("{:?}", s.html_url);
            Ok(())
        })
        .await?;

    Ok(())
}
