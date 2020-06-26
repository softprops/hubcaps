use futures::prelude::*;
use hubcaps::teams::{TeamMemberOptions, TeamMemberRole, TeamOptions};
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

    let org = "eb6cb83a-cf75-4e88-a11a-ce117467d8ae";
    let repo_name = "d18e3679-9830-40a9-8cf5-16602639b43e";

    println!("org teams");
    github
        .org(org)
        .teams()
        .iter()
        .try_for_each(|team| async move {
            println!("{:#?}", team);
            Ok(())
        })
        .await?;

    println!("repo teams");
    github
        .repo(org, repo_name)
        .teams()
        .iter()
        .try_for_each(|team| async move {
            println!("{:#?}", team);
            Ok(())
        })
        .await?;

    let new_team = github
        .org(org)
        .teams()
        .create(&TeamOptions {
            name: String::from("hi"),
            description: Some(String::from("there")),
            permission: None,
            privacy: Some(String::from("secret")),
        })
        .await?;
    println!("Created team: {:#?}", new_team);

    let team = github.org(org).teams().get(new_team.id);

    let updated_team = team
        .update(&TeamOptions {
            name: String::from("hello"),
            description: None,
            permission: None,
            privacy: None,
        })
        .await?;
    println!("Updated team: {:#?}", updated_team);

    println!(
        "Adding grahamc to the team: {:#?}",
        team.add_user(
            "grahamc",
            TeamMemberOptions {
                role: TeamMemberRole::Member,
            }
        )
        .await?
    );

    println!("members:");
    team.iter_members()
        .try_for_each(|member| async move {
            println!("{:#?}", member);
            Ok(())
        })
        .await?;

    println!(
        "Removing grahamc from the team: {:#?}",
        team.remove_user("grahamc").await?
    );

    let deleted_team = team.delete().await?;
    println!("Deleted team: {:#?}", deleted_team);

    Ok(())
}
