use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type RequestHeaders = HashMap<String, String>;
pub type Tags = Vec<String>;
pub type ScheduleId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestDocument {
    /// Request method
    pub method: String,
    /// Request url
    pub url: String,
    /// Optional request headers
    pub headers: Option<RequestHeaders>,
    /// Optional request body
    pub body: Option<String>,
    /// If set, overrides the default retry delay
    pub retry: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallbackDocument {
    /// Callback url to which status is posted
    pub url: String,
    /// Optional callback headers
    pub headers: Option<RequestHeaders>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleDocument {
    /// Unique identifier for schedule
    pub id: String,
    /// Optional tags to group schedules
    pub tags: Option<Tags>,
    /// Request to be executed on given schedule
    pub request: RequestDocument,
    /// Schedule in cron format
    /// https://crontab.guru/
    pub schedule: Option<String>,
    /// Schedule in ISO 8601 format
    pub schedule_at: Option<String>,
    /// Callback to be executed after request is executed, this is optional
    pub callback: Option<CallbackDocument>,
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// last successful run
    pub last_run: Option<chrono::DateTime<chrono::Utc>>,
    /// status
    #[serde(default = "ScheduleStatus::default")]
    pub status: ScheduleStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScheduleStatus {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "executing")]
    Executing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for ScheduleStatus {
    fn default() -> Self {
        Self::Scheduled
    }
}
