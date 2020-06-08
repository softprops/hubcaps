use hubcaps::notifications::ThreadListOptions;
use hubcaps::{Credentials, Github, Result};
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

            let opts = ThreadListOptions::builder().all(true).build();
            for thread in github.activity().notifications().list(&opts).await? {
                println!("{:#?}", thread);
                let subscription = github
                    .activity()
                    .notifications()
                    .get_subscription(thread.id)
                    .await;
                if let Ok(sub) = subscription {
                    println!("{:#?}", sub);
                }
            }

            // Mark all notifications as read.
            github.activity().notifications().mark_as_read(None).await?;

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
