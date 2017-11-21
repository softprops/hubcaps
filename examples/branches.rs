extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;
extern crate futures;

use std::env;

use futures::Stream;

use hubcaps::{Credentials, Github};
use hubcaps::branches::Protection;


use tokio_core::reactor::Core;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new().unwrap();
            let github = Github::new(
                format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );

            if let Err(err) = core.run(
                github
                    .repo("softprops", "hubcaps")
                    .branches()
                    .iter()
                    .for_each(|branch| Ok(println!("{:#?}", branch))),
            )
            {
                println!("err {:#?}", err)
            }

            match core.run(github.repo("softprops", "hubcaps").branches().get("master")) {
                Ok(branch) => println!("{:#?}", branch),
                Err(err) => println!("err {:#?}", err),
            }

            // protect master branch
            match core.run(github.repo("softprops", "hubcaps").branches().protection(
                "master",
                &Protection {
                    required_status_checks: None,
                    enforce_admins: false,
                    required_pull_request_reviews: None,
                    restrictions: None,
                },
            )) {
                Ok(pro) => println!("{:#?}", pro),
                Err(err) => println!("err {:#?}", err),
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
