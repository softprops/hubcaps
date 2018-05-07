extern crate env_logger;
#[macro_use(quick_main)]
extern crate error_chain;
extern crate hubcaps;
extern crate tokio_core;

use std::collections::HashMap;
use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};
use hubcaps::gists::{Content, GistOptions};

quick_main!(run);

fn run() -> Result<()> {
    drop(env_logger::init());
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );

            // create new gist
            let mut files = HashMap::new();
            files.insert("file1", "Hello World");
            let options = GistOptions::new(Some("gist description"), false, files);
            let gist = core.run(github.gists().create(&options))?;
            println!("{:#?}", gist);

            // edit file1
            let mut files = HashMap::new();
            files.insert("file1", "Hello World!!");
            let options = GistOptions::new(None as Option<String>, false, files);
            let gist = core.run(github.gists().edit(&gist.id, &options))?;
            println!("{:#?}", gist);

            // rename file1 to file2
            let mut files = HashMap::new();
            files.insert(
                String::from("file1"),
                Content::new(Some("file2"), "Hello World!!"),
            );
            let options = GistOptions {
                description: None as Option<String>,
                public: None,
                files: files,
            };
            let gist = core.run(github.gists().edit(&gist.id, &options))?;
            println!("{:#?}", gist);

            // delete gist
            core.run(github.gists().delete(&gist.id))?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
