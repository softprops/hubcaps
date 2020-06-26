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
    if let Some(file) = github
        .repo("softprops", "hubcaps")
        .git()
        .tree("master", true)
        .await?
        .tree
        .iter()
        .find(|file| file.path == "README.md")
    {
        let blob = github
            .repo("softprops", "hubcaps")
            .git()
            .blob(file.sha.clone())
            .await?;
        println!("readme {:#?}", blob);
    }
    Ok(())
}
