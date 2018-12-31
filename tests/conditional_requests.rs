extern crate env_logger;
extern crate futures;
extern crate hubcaps;
extern crate hyper;
extern crate hyper_tls;
#[cfg(feature = "httpcache")]
#[macro_use]
extern crate log;
extern crate tokio;

#[cfg(feature = "httpcache")]
use std::env;

#[cfg(feature = "httpcache")]
use futures::{future, Stream};
#[cfg(feature = "httpcache")]
use hyper::Client;
#[cfg(feature = "httpcache")]
use hyper_tls::HttpsConnector;
#[cfg(feature = "httpcache")]
use tokio::runtime::Runtime;

#[cfg(feature = "httpcache")]
use hubcaps::http_cache::FileBasedCache;
#[cfg(feature = "httpcache")]
use hubcaps::repositories::UserRepoListOptions;
#[cfg(feature = "httpcache")]
use hubcaps::{Credentials, Error, Github, Result};

#[cfg(feature = "httpcache")]
mod testkit;

#[test]
#[cfg(feature = "httpcache")]
fn compare_counts() -> Result<()> {
    env_logger::init();

    let mut rt = Runtime::new()?;

    let agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let credentials = env::var("GITHUB_TOKEN").ok().map(Credentials::Token);
    let owner = "octocat";
    let per_page = 5;
    let repo_list_options = UserRepoListOptions::builder().per_page(per_page).build();

    info!("first get the total count of repos, without caching");

    let github = Github::new(agent, credentials.clone());
    let repos = github.user_repos(owner).iter(&repo_list_options);
    let total_count = rt.block_on(repos.fold(0, |acc, _repo| future::ok::<_, Error>(acc + 1)))?;

    // octocat current has 8 repos, so we set per_page to 5 to get 2 pages
    // but if octocat ends up having less than 5 repos, it'll be just one page
    // and therefore nullify this test, so we sanity check
    assert!(
        total_count > per_page,
        "test sanity check failed, total_count: {}, per_page: {}",
        total_count,
        per_page,
    );

    info!("then get the total count with a cache");

    let host = "https://api.github.com";
    let client = Client::builder().build(HttpsConnector::new(4).unwrap());
    let cache_path = testkit::test_home().join(".hubcaps/cache");
    let http_cache = Box::new(FileBasedCache::new(cache_path));
    let github: Github<_> = Github::custom(host, agent, credentials, client, http_cache);

    info!("first populate the cache");

    let repos = github.user_repos(owner).iter(&repo_list_options);
    let count1 = rt.block_on(repos.fold(0, |acc, _repo| future::ok::<_, Error>(acc + 1)))?;
    let status1 = rt.block_on(github.rate_limit().get())?;

    info!("then retrieve via the cache");

    let repos = github.user_repos(owner).iter(&repo_list_options);
    let count2 = rt.block_on(repos.fold(0, |acc, _repo| future::ok::<_, Error>(acc + 1)))?;
    let status2 = rt.block_on(github.rate_limit().get())?;

    info!("and compare the counts");

    assert_eq!(count1, count2);

    info!("and while we're at it, compare that caching mitigates rate limiting");

    let rem1 = status1.resources.core.remaining;
    let rem2 = status2.resources.core.remaining;
    assert_eq!(rem1, rem2);

    Ok(())
}
