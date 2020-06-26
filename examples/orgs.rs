use hubcaps::repositories::{OrgRepoType, OrganizationRepoListOptions};
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

    let options = OrganizationRepoListOptions::builder()
        .repo_type(OrgRepoType::Forks)
        .build();

    println!("Forks in the rust-lang organization:");

    for repo in github.org_repos("rust-lang").list(&options).await? {
        println!("{}", repo.name)
    }

    println!();

    println!("My organizations:");
    for org in github.orgs().list().await? {
        println!("{}", org.login)
    }

    println!();

    println!("softprops' organizations:");
    for org in github.user_orgs("softprops").list().await? {
        println!("{}", org.login)
    }
    Ok(())
}
