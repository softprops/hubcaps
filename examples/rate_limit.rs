extern crate env_logger;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use tokio_core::reactor::Core;

use hubcaps::{Github, Result};

fn main() -> Result<()> {
    drop(env_logger::init());
    let mut core = Core::new()?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        None,
        &core.handle(),
    );
    let status = core.run(github.rate_limit().get())?;
    println!("{:#?}", status);
    Ok(())
}
