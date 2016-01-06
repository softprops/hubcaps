extern crate hyper;
extern crate hubcaps;

use hyper::Client;
use hubcaps::{Github, IssueListOptions};
use std::env;

fn main() {
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let client = Client::new();
            let github = Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
            &client,
                                     Some(token));
            let repo = github.repo("softprops", "hubcaps");
            let issues = repo.issues();
            for issue in issues.list(&IssueListOptions::builder().build()).unwrap() {
                println!("#{} {}", issue.number, issue.title);
            }
        },
        _ => println!("example missing GITHUB_TOKEN")
    }
}
