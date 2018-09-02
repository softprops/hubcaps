extern crate env_logger;
extern crate hubcaps;
extern crate tokio;

use tokio::runtime::Runtime;

use hubcaps::{Github, Result};

fn main() -> Result<()> {
    drop(env_logger::init());

    let mut rt = Runtime::new()?;

    let agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let credentials = None;
    let github = Github::new(agent, credentials);

    let _repos = rt.block_on(github.user_repos("dwijnand").list(&Default::default()))?;
    let status1 = rt.block_on(github.rate_limit().get())?;
    println!("rate limit status:\n{:#?}", status1);

    let _repos = rt.block_on(github.user_repos("dwijnand").list(&Default::default()))?;
    let status2 = rt.block_on(github.rate_limit().get())?;
    println!("rate limit status:\n{:#?}", status2);

    let rem1 = status1.resources.core.remaining;
    let rem2 = status2.resources.core.remaining;

    assert_eq!(rem1, rem2);

    Ok(())
}
