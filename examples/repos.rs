extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hubcaps::{Credentials, Github};
use std::env;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github =
                Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                            Client::with_connector(HttpsConnector::new(NativeTlsClient::new()
                                .unwrap())),
                            Credentials::Token(token));
            for repo in github.user_repos("softprops").list(&Default::default()).unwrap() {
                println!("{}", repo.name);
                for (language, bytes_of_code) in &repo.languages(&github).unwrap() {
                    println!("{}: {} bytes", language, bytes_of_code);
                }
                println!("");
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
