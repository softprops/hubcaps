use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

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
            let repo = github.repo("softprops", "hubcat");
            let pulls = repo.pulls();
            rt.block_on(pulls.iter(&Default::default()).for_each(|pull| {
                println!("{:#?}", pull);
                Ok(())
            }))?;

            println!("comments");
            for c in rt.block_on(
                github
                    .repo("softprops", "hubcaps")
                    .pulls()
                    .get(28)
                    .comments()
                    .list(&Default::default()),
            )? {
                println!("{:#?}", c);
            }

            println!("commits");
            rt.block_on(
                github
                    .repo("softprops", "hubcaps")
                    .pulls()
                    .get(28)
                    .commits()
                    .iter()
                    .for_each(|c| {
                        println!("{:#?}", c);
                        Ok(())
                    }),
            )?;

            println!("review requests");
            println!("{:#?}",
                     rt.block_on(
                         github
                             .repo("softprops", "hubcaps")
                             .pulls()
                             .get(190)
                             .review_requests()
                             .get()
                     )?);
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
