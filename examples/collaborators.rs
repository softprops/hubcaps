use hubcaps::{self, Credentials, Github, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;

            println!("My organizations:");
            println!("");

            for org in github.orgs().list().await? {
                println!("{}", org.login);
                println!("=============");
                println!("Repos:");

                for repo in github
                    .org_repos(&org.login[..])
                    .list(&Default::default())
                    .await?
                {
                    println!("* {}", repo.name);

                    // If you have push permissions on an org, you can list collaborators.
                    // Otherwise, don't print them.
                    if let Ok(collabs) = github
                        .repo(&org.login[..], &repo.name[..])
                        .collaborators()
                        .list()
                        .await
                    {
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
