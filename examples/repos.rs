use std::env;

use futures::prelude::*;
use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = &Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;
            rt.block_on(
                github
                    .user_repos("softprops")
                    .iter(&Default::default())
                    .try_for_each(move |repo| async move {
                        println!("{}", repo.name);
                        let f = repo.languages(github.clone()).map_ok(|langs| {
                            for (language, bytes_of_code) in langs {
                                println!("{}: {} bytes", language, bytes_of_code)
                            }
                        });
                        tokio::spawn(f.map(|_| ()));
                        Ok(())
                    }),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
