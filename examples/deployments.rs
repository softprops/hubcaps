extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::{Credentials, Github};
use std::env;

fn main() {
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                                     Client::new(),
                                     Credentials::Token(token));
            let repo = github.repo("softprops", "hubcaps");
            let deployments = repo.deployments();
            // let deploy = deployments.create(&DeploymentOptions::builder("master")
            // .payload("this is the payload".to_owned()).build());
            // println!("{:?}", deploy);
            for d in deployments.list(&Default::default()).unwrap() {
                println!("{:#?}", d)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
