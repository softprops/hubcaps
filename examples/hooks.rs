use hubcaps::hooks::{HookCreateOptions, WebHookContentType};
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )?;
    let repo = github.repo("softprops", "hubcaps");
    let hook = repo
        .hooks()
        .create(
            &HookCreateOptions::web()
                .url("http://localhost:8080")
                .content_type(WebHookContentType::Json)
                .build(),
        )
        .await;
    println!("{:#?}", hook);
    let hooks = repo.hooks();
    for hook in hooks.list().await? {
        println!("{:#?}", hook)
    }
    Ok(())
}
