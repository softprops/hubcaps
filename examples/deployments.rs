extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github};

fn main() {
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new().unwrap();
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            let repo = github.repo("softprops", "hubcaps");
            let deployments = repo.deployments();
            // let deploy = deployments.create(&DeploymentOptions::builder("master")
            // .payload("this is the payload".to_owned()).build());
            // println!("{:?}", deploy);
            for d in core.run(deployments.list(&Default::default())).unwrap() {
                println!("{:#?}", d)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
