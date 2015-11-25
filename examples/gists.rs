extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::{GistReq, Github};
use std::collections::HashMap;

fn main() {
    let client = Client::new();
    let github = Github::new(
        format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
        &client,
        Some(env!("GITHUB_TOKEN"))
    );
    let mut files = HashMap::new();
    files.insert("thename", "the contents");
    let gist = github.gists().create(
        &GistReq::new(
            None, false, files
        )
    ).unwrap();
    println!("{:?}", gist)
}
