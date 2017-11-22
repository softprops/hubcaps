extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

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
            for gist in core.run(github.gists().list(&Default::default())).unwrap() {
                println!("{:#?}", gist)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
