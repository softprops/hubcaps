use std::env;

use futures::future;
use futures::TryStreamExt;

use hubcaps::branches::Protection;
use hubcaps::{Credentials, Github, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;

            if let Err(err) = github
                .repo("softprops", "hubcaps")
                .branches()
                .iter()
                .await
                .try_for_each(|branch| {
                    println!("{:#?}", branch);
                    future::ok(())
                })
                .await
            {
                println!("err {:#?}", err)
            }

            match github
                .repo("softprops", "hubcaps")
                .branches()
                .get("master")
                .await
            {
                Ok(branch) => println!("{:#?}", branch),
                Err(err) => println!("err {:#?}", err),
            }

            // protect master branch
            match github
                .repo("softprops", "hubcaps")
                .branches()
                .protection(
                    "master",
                    &Protection {
                        required_status_checks: None,
                        enforce_admins: false,
                        required_pull_request_reviews: None,
                        restrictions: None,
                    },
                )
                .await
            {
                Ok(pro) => println!("{:#?}", pro),
                Err(err) => println!("err {:#?}", err),
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
