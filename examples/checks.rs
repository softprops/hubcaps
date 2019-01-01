extern crate pretty_env_logger;
extern crate hubcaps;
extern crate jsonwebtoken as jwt;
extern crate serde_json;
extern crate tokio;

use std::env;
use std::fs::File;
use std::io::Read;

use tokio::runtime::Runtime;

use hubcaps::checks::{
    Action, Annotation, AnnotationLevel, CheckRunOptions, Conclusion, Image, Output,
};
use hubcaps::git::GetReferenceResponse;
use hubcaps::{Credentials, Github, InstallationTokenGenerator, JWTCredentials, Result};

fn var(name: &str) -> Result<String> {
    if let Some(v) = env::var(name).ok() {
        Ok(v)
    } else {
        Err(format!("example missing {}", name).into())
    }
}

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn main() -> Result<()> {
    pretty_env_logger::init();
    let key_file = var("GH_APP_KEY")?;
    let app_id = var("GH_APP_ID")?;
    let user_name = var("GH_USERNAME")?;
    let repo = var("GH_REPO")?;
    let branch = var("GH_BRANCH")?;
    let mut rt = Runtime::new()?;

    let mut key = Vec::new();
    File::open(&key_file)?.read_to_end(&mut key)?;
    let cred = JWTCredentials::new(app_id.parse().expect("Bad GH_APP_ID"), key)?;

    let mut github = Github::new(USER_AGENT, Credentials::JWT(cred.clone()));
    let installation = rt
        .block_on(
            github
                .app()
                .find_repo_installation(user_name.clone(), repo.clone()),
        ).unwrap();

    github.set_credentials(Credentials::InstallationToken(
        InstallationTokenGenerator::new(installation.id, cred),
    ));

    let repo = github.repo(user_name, repo);
    let reference = repo.git().reference(format!("heads/{}", &branch));
    let sha = match rt.block_on(reference).unwrap() {
        GetReferenceResponse::Exact(r) => r.object.sha,
        GetReferenceResponse::StartWith(_) => panic!("Branch {} not found", &branch),
    };

    let checks = repo.checkruns();
    let options = &CheckRunOptions {
        actions: Some(vec![
            Action {
                description: "click to do a thing".to_string(),
                identifier: "the thing".to_string(),
                label: "nix-build -A pkgA".to_string(),
            },
            Action {
                description: "click to do a different thing".to_string(),
                identifier: "the different thing".to_string(),
                label: "nix-build -A pkgB".to_string(),
            },
        ]),
        completed_at: Some("2018-01-01T01:01:01Z".to_string()),
        started_at: Some("2018-08-01T01:01:01Z".to_string()),
        conclusion: Some(Conclusion::Neutral),
        details_url: Some("https://nix.ci/status/hi".to_string()),
        external_id: Some("heyyy".to_string()),
        head_sha: sha,
        name: "nix-build . -A pkgA".to_string(),
        output: Some(Output {
            annotations: Some(vec![
                Annotation {
                    annotation_level: AnnotationLevel::Warning,
                    start_line: 4,
                    end_line: 4,
                    start_column: Some(4),
                    end_column: Some(6),
                    message: "Trailing whitespace".to_string(),
                    path: "bogus".to_string(),
                    raw_details: "".to_string(),
                    title: "Whitespace".to_string(),
                },
                Annotation {
                    annotation_level: AnnotationLevel::Warning,
                    start_line: 7,
                    end_line: 7,
                    start_column: Some(4),
                    end_column: Some(8),
                    message: "not sure you meant this letter".to_string(),
                    path: "bogus".to_string(),
                    raw_details: "rawdeetshere\n  is\n   some\n    text".to_string(),
                    title: "hiiii".to_string(),
                },
            ]),
            images: Some(vec![Image {
                alt: "alt text".to_string(),
                caption: Some("caption text".to_string()),
                image_url: "https://nix.ci/nix.ci.svg".to_string(),
            }]),
            summary: "build failed".to_string(),
            text: Some("texthere\n  is\n   some\n    text".to_string()),
            title: "build failed".to_string(),
        }),
        status: None,
    };

    println!("{}", serde_json::to_string(options).unwrap());
    println!("{:?}", rt.block_on(checks.create(options)));
    Ok(())
}
