use reqwest::Client;

use hubcaps::{Github, HttpCache, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
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
    let status2 = github.rate_limit().get().await?;

    let rem1 = status1.resources.core.remaining;
    let rem2 = status2.resources.core.remaining;

    assert_eq!(rem1, rem2);

    Ok(())
}
