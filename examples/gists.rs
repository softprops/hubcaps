extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::{GistReq, Github};
use std::collections::HashMap;
use std::env;

fn main() {
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let client = Client::new();
            let github = Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                                     &client,
                                     Some(token));
            let mut files = HashMap::new();
            files.insert("thename", "the contents");
            let gist = github.gists()
                             .create(&GistReq::builder(files).public(false).build())
                             .unwrap();
            println!("{:?}", gist)
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
