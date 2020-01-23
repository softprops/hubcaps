use std::env;

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
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
