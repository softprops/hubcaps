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
            for repo in github.user_repos("softprops").list(&Default::default()).unwrap() {
                println!("{}", repo.name);
                for (language, bytes_of_code) in &repo.languages(&github).unwrap() {
                    println!("{}: {} bytes", language, bytes_of_code);
                }
                println!("");
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
