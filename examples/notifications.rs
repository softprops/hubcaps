extern crate env_logger;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::notifications::ThreadListOptions;
use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );

            let opts = ThreadListOptions::builder().all(true).build();
            for thread in core.run(github.activity().notifications().list(&opts))? {
                println!("{:#?}", thread);
                let subscription = core.run(
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
            core.run(github.activity().notifications().mark_as_read(None))?;

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
