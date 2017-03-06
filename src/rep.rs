//! Rust representations of Github API data structures
use users::User;


#[derive(Debug, Deserialize)]
pub struct Commit {
    pub label: String,
    #[serde(rename="ref")]
    pub commit_ref: String,
    pub sha: String,
    pub user: User, //    pub repo: Option<Repo>,
}

#[derive(Debug, Deserialize)]
pub struct UserStamp {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitRef {
    pub url: String,
    pub sha: String,
}
