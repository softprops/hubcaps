extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github};
use hubcaps::repositories::{OrgRepoType, OrganizationRepoListOptions};

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new().unwrap();
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );

            let options = OrganizationRepoListOptions::builder()
                .repo_type(OrgRepoType::Forks)
                .build();

            println!("Forks in the rust-lang organization:");

            for repo in core.run(github.org_repos("rust-lang").list(&options))
                .unwrap()
            {
                println!("{}", repo.name)
            }

            println!("");

            println!("My organizations:");
            for org in core.run(github.orgs().list()).unwrap() {
                println!("{}", org.login)
            }

            println!("");

            println!("softprops' organizations:");
            for org in core.run(github.user_orgs("softprops").list()).unwrap() {
                println!("{}", org.login)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
