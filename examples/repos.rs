extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::Github;
use std::env;

fn main() {
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let client = Client::new();
            let github = Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                                     &client,
                                     Some(token));
            for repo in github.repos().list().unwrap() {
                println!("{:#?}", repo)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
