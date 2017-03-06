extern crate env_logger;
extern crate hyper;
extern crate hyper_native_tls;
extern crate hubcaps;

use hyper::net::HttpsConnector;
use hyper::Client;
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
            for gist in github.gists().list(&Default::default()).unwrap() {
                println!("{:#?}", gist)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
