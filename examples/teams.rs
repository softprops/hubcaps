extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate futures;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github};

fn main() {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new().unwrap();
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            println!("org teams");
            core.run(github.org("meetup").teams().iter().for_each(|team| {
                Ok(println!("{:#?}", team))
            })).unwrap();
            println!("repo teams");
            core.run(
                github
                    .repo("meetup", "k8s-nginx-dogstats")
                    .teams()
                    .iter()
                    .for_each(|team| Ok(println!("{:#?}", team))),
            ).unwrap()
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
