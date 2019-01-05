use std::env;

use futures::Stream;
use tokio::runtime::Runtime;

use hubcaps::teams::{TeamMemberOptions, TeamMemberRole, TeamOptions};
use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );

            let org = "eb6cb83a-cf75-4e88-a11a-ce117467d8ae";
            let repo_name = "d18e3679-9830-40a9-8cf5-16602639b43e";

            println!("org teams");
            rt.block_on(github.org(org).teams().iter().for_each(|team| {
                println!("{:#?}", team);
                Ok(())
            }))
            .unwrap_or_else(|e| println!("error: {:#?}", e));

            println!("repo teams");
            rt.block_on(github.repo(org, repo_name).teams().iter().for_each(|team| {
                println!("{:#?}", team);
                Ok(())
            }))
            .unwrap_or_else(|e| println!("error: {:#?}", e));

            let new_team = rt.block_on(github.org(org).teams().create(&TeamOptions {
                name: String::from("hi"),
                description: Some(String::from("there")),
                permission: None,
                privacy: Some(String::from("secret")),
            }))?;
            println!("Created team: {:#?}", new_team);

            let team = github.org(org).teams().get(new_team.id);

            let updated_team = rt.block_on(team.update(&TeamOptions {
                name: String::from("hello"),
                description: None,
                permission: None,
                privacy: None,
            }))?;
            println!("Updated team: {:#?}", updated_team);

            println!(
                "Adding grahamc to the team: {:#?}",
                rt.block_on(team.add_user(
                    "grahamc",
                    TeamMemberOptions {
                        role: TeamMemberRole::Member,
                    }
                ))
            );

            println!("members:");
            rt.block_on(team.iter_members().for_each(|member| {
                println!("{:#?}", member);
                Ok(())
            }))
            .unwrap_or_else(|e| println!("error: {:#?}", e));

            println!(
                "Removing grahamc from the team: {:#?}",
                rt.block_on(team.remove_user("grahamc"))
            );

            let deleted_team = rt.block_on(team.delete())?;
            println!("Deleted team: {:#?}", deleted_team);

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
