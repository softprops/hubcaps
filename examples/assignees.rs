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
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
