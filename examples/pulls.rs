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
            let pulls = repo.pulls();
            for pull in pulls.list(&Default::default()).unwrap() {
                println!("{:#?}", pull);
            }

            println!("comments");
            for c in github.repo("softprops", "hubcaps")
                .pulls()
                .get(28)
                .comments()
                .list(&Default::default()).unwrap() {
                    println!("{:#?}", c);
                }

            println!("commits");
            for c in github.repo("softprops", "hubcaps")
                .pulls()
                .get(28)
                .commits()
                .iter().unwrap() {
                    println!("{:#?}", c);
                }
        },
        _ => println!("example missing GITHUB_TOKEN")
    }
}
