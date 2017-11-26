extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;
#[macro_use(quick_main)]
extern crate error_chain;

use std::env;

use futures::{Future, Stream};
use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Some(Credentials::Token(token)),
                &core.handle(),
            );
            let handle = core.handle();
            core.run(
                github
                    .user_repos("softprops")
                    .iter(&Default::default())
                    .for_each(move |repo| {
                        println!("{}", repo.name);
                        let f = repo.languages(github.clone()).map(|langs| {
                            for (language, bytes_of_code) in langs {
                                println!("{}: {} bytes", language, bytes_of_code)
                            }
                        });
                        handle.spawn(f.map_err(|_| ()));
                        Ok(())
                    }),
            )?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
