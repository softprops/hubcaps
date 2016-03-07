extern crate env_logger;
extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::{GistOptions, Github};
use std::collections::HashMap;
use std::env;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let client = Client::new();
            let github = Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                                     &client,
                                     Some(token));
            /*let mut files = HashMap::new();
            files.insert("thename", "the contents");
            let gist = github.gists()
                             .create(&GistOptions::builder(files).public(false).build())
                             .unwrap();
            println!("{:#?}", gist)*/
            for gist in github.gists().list().unwrap() {
                println!("{:#?}", gist)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
