use futures::prelude::*;
use hubcaps::branches::Protection;
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )?;

    if let Err(err) = github
        .repo("softprops", "hubcaps")
        .branches()
        .iter()
        .try_for_each(|branch| async move {
            println!("{:#?}", branch);
            Ok(())
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
