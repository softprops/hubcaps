extern crate pretty_env_logger;
extern crate hubcaps;
extern crate tokio;

use std::env;

use tokio::runtime::Runtime;

use hubcaps::notifications::ThreadListOptions;
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

            let opts = ThreadListOptions::builder().all(true).build();
            for thread in rt.block_on(github.activity().notifications().list(&opts))? {
                println!("{:#?}", thread);
                let subscription = rt.block_on(
                    github
                        .activity()
                        .notifications()
                        .get_subscription(thread.id),
                );
                if let Ok(sub) = subscription {
                    println!("{:#?}", sub);
                }
            }

            // Mark all notifications as read.
            rt.block_on(github.activity().notifications().mark_as_read(None))?;

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
