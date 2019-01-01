extern crate pretty_env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio;

use tokio::runtime::Runtime;

use hubcaps::{Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    let mut rt = Runtime::new()?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        None,
    );
    let status = rt.block_on(github.rate_limit().get())?;
    println!("{:#?}", status);
    Ok(())
}
