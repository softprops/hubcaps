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

            println!("My organizations:");
            println!("");

            for org in github.orgs().list().unwrap() {
                println!("{}", org.login);
                println!("=============");
                println!("Repos:");

                for repo in github.org_repos(&org.login[..]).list(&Default::default()).unwrap() {
                    println!("* {}", repo.name);

                    // If you have push permissions on an org, you can list collaborators.
                    // Otherwise, don't print them.
                    if let Ok(collabs) = github.repo(
                            &org.login[..],
                            &repo.name[..]
                        ).collaborators().list() {
                        println!("  * Collaborators: {}", collabs.into_iter().map(|c| {
                            c.login
                        }).collect::<Vec<_>>().join(", "));
                    }
                }
                println!("")
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
