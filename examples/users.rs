extern crate env_logger;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github};

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new().unwrap();
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            match core.run(github.users().authenticated()) {
                Ok(me) => println!("{:#?}", me),
                Err(err) => println!("err {:#?}", err),
            }

            match core.run(github.users().get(env::var("GH_USERNAME").ok().unwrap_or(
                "bors".into(),
            ))) {
                Ok(user) => println!("{:#?}", user),
                Err(err) => println!("err {:#?}", err),
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
