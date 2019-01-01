extern crate pretty_env_logger;
extern crate hubcaps;
extern crate tokio;

use std::env;

use tokio::runtime::Runtime;

use hubcaps::repositories::{OrgRepoType, OrganizationRepoListOptions};
use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );

            let options = OrganizationRepoListOptions::builder()
                .repo_type(OrgRepoType::Forks)
                .build();

            println!("Forks in the rust-lang organization:");

            for repo in rt.block_on(github.org_repos("rust-lang").list(&options))? {
                println!("{}", repo.name)
            }

            println!("");

            println!("My organizations:");
            for org in rt.block_on(github.orgs().list())? {
                println!("{}", org.login)
            }

            println!("");

            println!("softprops' organizations:");
            for org in rt.block_on(github.user_orgs("softprops").list())? {
                println!("{}", org.login)
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
