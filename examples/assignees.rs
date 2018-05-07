extern crate env_logger;
#[macro_use(quick_main)]
extern crate error_chain;
extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github, Result};

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
      let pull = core.run(
        github
          .repo("softprops", "hubcaps")
          .pulls()
          .get(122)
          .assignees()
          .add(vec!["softprops"]),
      )?;
      println!("{:#?}", pull);

      let issue = core.run(
        github
          .repo("softprops", "hubcaps")
          .issues()
          .get(125)
          .assignees()
          .add(vec!["softprops"]),
      )?;
      Ok(println!("{:#?}", issue))
    }
    _ => Err("example missing GITHUB_TOKEN".into()),
  }
}
