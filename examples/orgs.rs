extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hubcaps::{Credentials, Github};
use hubcaps::repositories::{OrgRepoType, OrganizationRepoListOptions};
use std::env;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github =
                Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                            Client::with_connector(HttpsConnector::new(NativeTlsClient::new()
                                .unwrap())),
                            Credentials::Token(token));

            let options =
                OrganizationRepoListOptions::builder().repo_type(OrgRepoType::Forks).build();

            println!("Forks in the rust-lang organization:");

            for repo in github.org_repos("rust-lang").list(&options).unwrap() {
                println!("{}", repo.name)
            }

            println!("");

            println!("My organizations:");
            for org in github.orgs().list().unwrap() {
                println!("{}", org.login)
            }

            println!("");

            println!("softprops' organizations:");
            for org in github.user_orgs("softprops").list().unwrap() {
                println!("{}", org.login)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
