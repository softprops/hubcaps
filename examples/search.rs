extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hubcaps::{Credentials, Github};
use hubcaps::search::SearchIssuesOptions;
use std::env;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github =
                Github::new(
                    format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                    Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap())),
                    Credentials::Token(token),
                );
            for issue in github
                .search()
                .issues()
                .iter(
                    "user:softprops",
                    &SearchIssuesOptions::builder().per_page(1).build(),
                )
                .unwrap()
            {
                println!("{}", issue.title);
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
