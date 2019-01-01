extern crate pretty_env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio;

use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::branches::Protection;
use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );

            if let Err(err) = rt.block_on(
                github
                    .repo("softprops", "hubcaps")
                    .branches()
                    .iter()
                    .for_each(|branch| {
                        println!("{:#?}", branch);
                        Ok(())
                    }),
            ) {
                println!("err {:#?}", err)
            }

            match rt.block_on(github.repo("softprops", "hubcaps").branches().get("master")) {
                Ok(branch) => println!("{:#?}", branch),
                Err(err) => println!("err {:#?}", err),
            }

            // protect master branch
            match rt.block_on(github.repo("softprops", "hubcaps").branches().protection(
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
