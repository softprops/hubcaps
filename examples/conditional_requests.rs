#[cfg(feature = "httpcache")]
use tokio::runtime::Runtime;

use hubcaps::Result;

#[cfg(feature = "httpcache")]
use hubcaps::{Github, HttpCache};

fn main() -> Result<()> {
    pretty_env_logger::init();

    #[cfg(not(feature = "httpcache"))]
    {
        println!("rerun this example with `cargo run --no-default-features --features default-tls,httpcache --example conditional_requests`");
        Ok(())
    }

    #[cfg(feature = "httpcache")]
    {
        let mut rt = Runtime::new()?;

        let http_cache = HttpCache::in_home_dir();
        let github = Github::builder().cache(http_cache).build()?;

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
