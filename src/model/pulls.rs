use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequest {
    pub number: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl PullRequest {
    pub fn lead_time(self) -> Duration {
        if self.merged_at.is_some() {
            self.merged_at.unwrap() - self.created_at
        } else {
            self.closed_at.unwrap() - self.created_at
        }
    }
}
