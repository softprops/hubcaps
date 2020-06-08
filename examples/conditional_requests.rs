use hubcaps::Result;
#[cfg(feature = "httpcache")]
use hubcaps::{Github, HttpCache};
#[cfg(feature = "httpcache")]
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    #[cfg(not(feature = "httpcache"))]
    {
        println!("rerun this example with `cargo run --no-default-features --features default-tls,httpcache --example conditional_requests`");
        Ok(())
    }

    #[cfg(feature = "httpcache")]
    {
        let host = "https://api.github.com";
        let agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        let client = Client::builder().build()?;
        let http_cache = HttpCache::in_home_dir();
        let github = Github::custom(host, agent, None, client, http_cache);

        let _repos = github
            .user_repos("dwijnand")
            .list(&Default::default())
            .await?;
        let status1 = github.rate_limit().get().await?;

        let _repos = github
            .user_repos("dwijnand")
            .list(&Default::default())
            .await?;
        let status2 = github.rate_limit().get()?;

        let rem1 = status1.resources.core.remaining;
        let rem2 = status2.resources.core.remaining;

        assert_eq!(rem1, rem2);

        Ok(())
    }
}
