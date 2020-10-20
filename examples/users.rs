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

    match github.users().authenticated_emails().await {
        Ok(emails) => println!("{:#?}", emails),
        Err(err) => println!("err {:#?}", err),
    }

    Ok(())
}
