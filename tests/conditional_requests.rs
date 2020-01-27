#[cfg(feature = "httpcache")]
use {
    futures::{future, TryStreamExt},
    hubcaps::http_cache::FileBasedCache,
    hubcaps::repositories::UserRepoListOptions,
    hubcaps::{Credentials, Github, Result},
    log::info,
    reqwest::Client,
    std::env,
};

#[cfg(feature = "httpcache")]
mod testkit;

#[tokio::test]
#[cfg(feature = "httpcache")]
async fn compare_counts() -> Result<()> {
    pretty_env_logger::init();

    let agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let credentials = match env::var("GITHUB_TOKEN").ok() {
        Some(token) => Some(Credentials::Token(token)),
        None => {
            if env::var("CI") == Ok(String::from("true")) {
                println!("No GITHUB_TOKEN env var in CI, skipping test");
                return Ok(());
            } else {
                None
            }
        }
    };
    let owner = "octocat";
    let per_page = 5;
    let repo_list_options = UserRepoListOptions::builder().per_page(per_page).build();

    info!("first get the total count of repos, without caching");

    let github = Github::new(agent, credentials.clone())?;
    let repos = github.user_repos(owner).iter(&repo_list_options).await;
    let total_count = repos.try_fold(0, |acc, _repo| future::ok(acc + 1)).await?;

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
    let client = Client::builder().build()?;
    let cache_path = testkit::test_home().join(".hubcaps/cache");
    let http_cache = Box::new(FileBasedCache::new(cache_path));
    let github = Github::custom(host, agent, credentials, client, http_cache);

    info!("first populate the cache");

    let repos = github.user_repos(owner).iter(&repo_list_options).await;
    let count1 = repos.try_fold(0, |acc, _repo| future::ok(acc + 1)).await?;
    let status1 = github.rate_limit().get().await?;

    info!("then retrieve via the cache");

    let repos = github.user_repos(owner).iter(&repo_list_options).await;
    let count2 = repos.try_fold(0, |acc, _repo| future::ok(acc + 1)).await?;
    let status2 = github.rate_limit().get().await?;

    info!("and compare the counts");

    assert_eq!(count1, count2);

    info!("and while we're at it, compare that caching mitigates rate limiting");

    let rem1 = status1.resources.core.remaining;
    let rem2 = status2.resources.core.remaining;
    assert_eq!(rem1, rem2);

    Ok(())
}
