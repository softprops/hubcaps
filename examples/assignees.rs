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
    let pull = github
        .repo("softprops", "hubcaps")
        .pulls()
        .get(122)
        .assignees()
        .add(vec!["softprops"])
        .await?;
    println!("{:#?}", pull);

    let issue = github
        .repo("softprops", "hubcaps")
        .issues()
        .get(125)
        .assignees()
        .add(vec!["softprops"])
        .await?;
    println!("{:#?}", issue);
    Ok(())
}
