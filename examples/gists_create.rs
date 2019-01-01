extern crate pretty_env_logger;
extern crate hubcaps;
extern crate tokio;

use std::collections::HashMap;
use std::env;

use tokio::runtime::Runtime;

use hubcaps::gists::{Content, GistOptions};
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

            // create new gist
            let mut files = HashMap::new();
            files.insert("file1", "Hello World");
            let options = GistOptions::new(Some("gist description"), false, files);
            let gist = rt.block_on(github.gists().create(&options))?;
            println!("{:#?}", gist);

            // edit file1
            let mut files = HashMap::new();
            files.insert("file1", "Hello World!!");
            let options = GistOptions::new(None as Option<String>, false, files);
            let gist = rt.block_on(github.gists().edit(&gist.id, &options))?;
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
            let gist = rt.block_on(github.gists().edit(&gist.id, &options))?;
            println!("{:#?}", gist);

            // delete gist
            rt.block_on(github.gists().delete(&gist.id))?;
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
