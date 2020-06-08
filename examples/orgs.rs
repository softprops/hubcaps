use hubcaps::repositories::{OrgRepoType, OrganizationRepoListOptions};
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

            let options = OrganizationRepoListOptions::builder()
                .repo_type(OrgRepoType::Forks)
                .build();

            println!("Forks in the rust-lang organization:");

            for repo in github.org_repos("rust-lang").list(&options).await? {
                println!("{}", repo.name)
            }

            println!("");

            println!("My organizations:");
            for org in github.orgs().list().await? {
                println!("{}", org.login)
            }

            println!("");

            println!("softprops' organizations:");
            for org in github.user_orgs("softprops").list().await? {
                println!("{}", org.login)
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
