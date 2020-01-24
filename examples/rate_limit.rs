use hubcaps::{Github, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        None,
    )?;
    let status = github.rate_limit().get().await?;
    println!("{:#?}", status);
    Ok(())
}
