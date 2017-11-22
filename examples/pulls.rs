extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use futures::Stream;
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
            let repo = github.repo("softprops", "hubcat");
            let pulls = repo.pulls();
            core.run(pulls.iter(&Default::default()).for_each(|pull| {
                Ok(println!("{:#?}", pull))
            })).unwrap();

            println!("comments");
            for c in core.run(
                github
                    .repo("softprops", "hubcaps")
                    .pulls()
                    .get(28)
                    .comments()
                    .list(&Default::default()),
            ).unwrap()
            {
                println!("{:#?}", c);
            }

            println!("commits");
            core.run(
                github
                    .repo("softprops", "hubcaps")
                    .pulls()
                    .get(28)
                    .commits()
                    .iter()
                    .for_each(|c| Ok(println!("{:#?}", c))),
            ).unwrap()
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
