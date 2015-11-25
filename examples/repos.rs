extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::Github;

fn main() {
    let client = Client::new();
    let github = Github::new(
        format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
        &client,
        Some(env!("GITHUB_TOKEN"))
    );
    for repo in github.repos().list().unwrap() {
        println!("{:?}", repo)
    }
}
