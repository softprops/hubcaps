use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;
            let repo = github.repo("softprops", "hubcaps");
            let deployments = repo.deployments();
            // let deploy = deployments.create(&DeploymentOptions::builder("master")
            // .payload("this is the payload".to_owned()).build());
            // println!("{:?}", deploy);
            for d in deployments.list(&Default::default()).await? {
                println!("{:#?}", d)
            }
            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
