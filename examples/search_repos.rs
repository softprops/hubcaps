extern crate env_logger;
#[macro_use(quick_main)]
extern crate error_chain;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};
use hubcaps::search::SearchReposOptions;

quick_main!(run);

fn run() -> Result<()> {
  drop(env_logger::init());
  match env::var("GITHUB_TOKEN").ok() {
    Some(token) => {
      let mut core = Core::new()?;
      let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
        &core.handle(),
      );
      println!("repo search results");
      // https://developer.github.com/v3/search/#parameters
      core.run(
        github
          .search()
          .repos()
          .iter(
            "user:softprops hubcaps",
            &SearchReposOptions::builder().per_page(100).build(),
          )
          .for_each(|repo| Ok(println!("{}", repo.full_name))),
      )?;
      Ok(())
    }
    _ => Err("example missing GITHUB_TOKEN".into()),
  }
}
