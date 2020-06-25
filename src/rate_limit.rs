//! Rate Limit interface
use crate::{Future, Github};
use serde::Deserialize;

pub struct RateLimit {
    github: Github,
}

impl RateLimit {
    #[doc(hidden)]
    pub fn new(github: Github) -> Self {
        Self { github }
    }

    /// https://developer.github.com/v3/rate_limit/#get-your-current-rate-limit-status
    pub fn get(&self) -> Future<RateLimitStatus> {
        self.github.get("/rate_limit")
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct RateLimitStatus {
    pub resources: RateLimitResourcesStatus,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitResourcesStatus {
    pub core: RateLimitResourceStatus,
    pub search: RateLimitResourceStatus,
    pub graphql: RateLimitResourceStatus,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitResourceStatus {
    pub limit: u32,
    pub remaining: u32,
    pub reset: u32, // ideally something like std::time::Duration
}
