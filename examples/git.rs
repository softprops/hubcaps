extern crate env_logger;
#[macro_use(quick_main)]
extern crate error_chain;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

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
            for file in core.run(
                github
                    .repo("softprops", "hubcaps")
                    .git()
                    .tree("master", true),
            )?
                .tree
                .iter()
                .find(|file| file.path == "README.md")
            {
                let blob = core.run(
                    github
                        .repo("softprops", "hubcaps")
                        .git()
                        .blob(file.sha.clone()),
                )?;
                println!("readme {:#?}", blob);
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
