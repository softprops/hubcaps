extern crate env_logger;
extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::{Credentials, Github};
use std::env;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                                     Client::new(),
                                     Credentials::Token(token));
            let repo = github.repo("softprops", "hubcat");
            let issues = repo.issues();
            for issue in issues.list(&Default::default()).unwrap() {
                println!("{:#?}", issue)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
