extern crate env_logger;
#[macro_use(quick_main)]
extern crate error_chain;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};
use hubcaps::repositories::{OrgRepoType, OrganizationRepoListOptions};

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Some(Credentials::Token(token)),
                &core.handle(),
            );

            let options = OrganizationRepoListOptions::builder()
                .repo_type(OrgRepoType::Forks)
                .build();

            println!("Forks in the rust-lang organization:");

            for repo in core.run(github.org_repos("rust-lang").list(&options))? {
                println!("{}", repo.name)
            }

            println!("");

            println!("My organizations:");
            for org in core.run(github.orgs().list())? {
                println!("{}", org.login)
            }

            println!("");

            println!("softprops' organizations:");
            for org in core.run(github.user_orgs("softprops").list())? {
                println!("{}", org.login)
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
