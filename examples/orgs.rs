extern crate env_logger;
extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::{Credentials, Github, OrganizationRepoListOptions};
use hubcaps::repositories::OrgRepoType;
use std::env;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let client = Client::new();
            let github = Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                                     &client,
                                     Credentials::Token(token));

            let options = OrganizationRepoListOptions::builder().repo_type(OrgRepoType::Forks).build();

            println!("Forks in the rust-lang organization:");

            for repo in github.org_repos("rust-lang").list(&options).unwrap() {
                println!("{}", repo.name)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
