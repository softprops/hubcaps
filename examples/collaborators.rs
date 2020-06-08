use hubcaps;

use hubcaps::{Credentials, Github, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;

            println!("My organizations:");
            println!("");

            for org in rt.block_on(github.orgs().list())? {
                println!("{}", org.login);
                println!("=============");
                println!("Repos:");

                for repo in
                    rt.block_on(github.org_repos(&org.login[..]).list(&Default::default()))?
                {
                    println!("* {}", repo.name);

                    // If you have push permissions on an org, you can list collaborators.
                    // Otherwise, don't print them.
                    if let Ok(collabs) = rt.block_on(
                        github
                            .repo(&org.login[..], &repo.name[..])
                            .collaborators()
                            .list(),
                    ) {
                        println!(
                            "  * Collaborators: {}",
                            collabs
                                .into_iter()
                                .map(|c| { c.login })
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                }
                println!("")
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
