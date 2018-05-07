extern crate env_logger;
#[macro_use(quick_main)]
extern crate error_chain;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};
use hubcaps::branches::Protection;

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );

            if let Err(err) = core.run(
                github
                    .repo("softprops", "hubcaps")
                    .branches()
                    .iter()
                    .for_each(|branch| Ok(println!("{:#?}", branch))),
            ) {
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
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
