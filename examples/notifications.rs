use hubcaps::notifications::ThreadListOptions;
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
