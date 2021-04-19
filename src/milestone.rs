use serde::{Deserialize, Serialize};

use crate::users::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: u64,
    pub node_id: String,
    pub number: u64,
    pub title: String,
    pub description: Option<String>,
    pub creator: User,
    pub open_issues: u64,
    pub closed_issues: u64,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub due_on: Option<String>,
    pub closed_at: Option<String>,
}