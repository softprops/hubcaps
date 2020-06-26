use hubcaps::gists::{Content, GistOptions};
use hubcaps::{Credentials, Github};
use std::collections::HashMap;
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

    // create new gist
    let mut files = HashMap::new();
    files.insert("file1", "Hello World");
    let options = GistOptions::new(Some("gist description"), false, files);
    let gist = github.gists().create(&options).await?;
    println!("{:#?}", gist);

    // edit file1
    let mut files = HashMap::new();
    files.insert("file1", "Hello World!!");
    let options = GistOptions::new(None as Option<String>, false, files);
    let gist = github.gists().edit(&gist.id, &options).await?;
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
        files,
    };
    let gist = github.gists().edit(&gist.id, &options).await?;
    println!("{:#?}", gist);

    // delete gist
    github.gists().delete(&gist.id).await?;
    Ok(())
}
