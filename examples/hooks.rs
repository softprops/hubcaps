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
            let client = Client::new();
            let github = Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                                     &client,
                                     Credentials::Token(token));
            let repo = github.repo("softprops", "hubcat");
            let hooks = repo.hooks();
            for hook in hooks.list().unwrap() {
                println!("{:#?}", hook)
            }
        },
        _ => println!("example missing GITHUB_TOKEN")
    }
}
