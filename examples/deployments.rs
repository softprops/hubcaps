extern crate hyper;
extern crate hubcaps;
extern crate hyper_native_tls;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hubcaps::{Credentials, Github};
use std::env;

fn main() {
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github =
                Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                            Client::with_connector(HttpsConnector::new(NativeTlsClient::new()
                                                                           .unwrap())),
                            Credentials::Token(token));
            let repo = github.repo("softprops", "hubcaps");
            let deployments = repo.deployments();
            // let deploy = deployments.create(&DeploymentOptions::builder("master")
            // .payload("this is the payload".to_owned()).build());
            // println!("{:?}", deploy);
            for d in deployments.list(&Default::default()).unwrap() {
                println!("{:#?}", d)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
