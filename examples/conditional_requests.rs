extern crate pretty_env_logger;
extern crate hubcaps;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

#[cfg(feature = "httpcache")]
use hyper::Client;
#[cfg(feature = "httpcache")]
use hyper_tls::HttpsConnector;
#[cfg(feature = "httpcache")]
use tokio::runtime::Runtime;

use hubcaps::Result;

#[cfg(feature = "httpcache")]
use hubcaps::{Github, HttpCache};

fn main() -> Result<()> {
    pretty_env_logger::init();

    #[cfg(not(feature = "httpcache"))]
    {
        println!("rerun this example with `cargo run --no-default-features --features tls,httpcache --example conditional_requests`");
        Ok(())
    }

    #[cfg(feature = "httpcache")]
    {
        let mut rt = Runtime::new()?;

        let host = "https://api.github.com";
        let agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        let client = Client::builder().build(HttpsConnector::new(4).unwrap());
        let http_cache = HttpCache::in_home_dir();
        let github = Github::custom(host, agent, None, client, http_cache);

        let _repos = rt.block_on(github.user_repos("dwijnand").list(&Default::default()))?;
        let status1 = rt.block_on(github.rate_limit().get())?;

        let _repos = rt.block_on(github.user_repos("dwijnand").list(&Default::default()))?;
        let status2 = rt.block_on(github.rate_limit().get())?;

        let rem1 = status1.resources.core.remaining;
        let rem2 = status2.resources.core.remaining;

        assert_eq!(rem1, rem2);

        Ok(())
    }
}
