//! Organization Membership interface
use serde::Deserialize;

use crate::users::User;
use crate::{Github, Stream};

/// Provides access to membership operations available for an individual organization
pub struct OrgMembership {
    github: Github,
    org: String,
}

impl OrgMembership {
    #[doc(hidden)]
    pub fn new<O>(github: Github, org: O) -> Self
    where
        O: Into<String>,
    {
        Self {
            github,
            org: org.into(),
        }
    }

    /// Return a stream of all invitations for this repository
    ///
    /// See the [github docs](https://developer.github.com/v3/orgs/members/)
    /// for more information
    pub async fn invitations(&self) -> Stream<Invitation> {
        self.github
            .get_stream(&format!("/orgs/{}/invitations", self.org))
            .await
    }
}

#[derive(Debug, Deserialize)]
pub struct Invitation {
    pub id: u64,
    pub login: Option<String>,
    pub email: Option<String>,
    pub role: InvitedRole,
    pub created_at: String, // TODO: change to `DateTime`?
    pub inviter: User,
    pub team_count: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InvitedRole {
    DirectMember,
    Admin,
    BillingManager,
    HiringManager,
    Reinstate,
}
