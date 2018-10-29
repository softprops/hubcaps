extern crate env_logger;
extern crate hubcaps;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio::runtime::Runtime;

use hubcaps::{Github, HttpCache, Result};

fn main() -> Result<()> {
    env_logger::init();

    let mut rt = Runtime::new()?;

    let host = "https://api.github.com";
    let agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let credentials = None;
    let client = Client::builder().build(HttpsConnector::new(4).unwrap());
    let http_cache = HttpCache::in_home_dir();
    let github = Github::custom(host, agent, credentials, client, http_cache);

    let _repos = rt.block_on(github.user_repos("dwijnand").list(&Default::default()))?;
    let status1 = rt.block_on(github.rate_limit().get())?;

    let _repos = rt.block_on(github.user_repos("dwijnand").list(&Default::default()))?;
    let status2 = rt.block_on(github.rate_limit().get())?;

    let rem1 = status1.resources.core.remaining;
    let rem2 = status2.resources.core.remaining;

    assert_eq!(rem1, rem2);

    Ok(())
}
