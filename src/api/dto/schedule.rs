use crate::db::schema::{
    CallbackDocument, RequestHeaders, ScheduleDocument, ScheduleStatus, Tags,
};
use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallbackDto {
    pub url: String,
    pub headers: Option<RequestHeaders>,
}

impl From<CallbackDocument> for CallbackDto {
    fn from(document: CallbackDocument) -> Self {
        Self {
            url: document.url,
            headers: document.headers,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestDto {
    pub url: String,
    pub method: String,
    pub headers: Option<RequestHeaders>,
    pub body: Option<String>,
    pub retry: Option<Vec<u32>>,
}

impl From<crate::db::schema::RequestDocument> for RequestDto {
    fn from(document: crate::db::schema::RequestDocument) -> Self {
        Self {
            url: document.url,
            method: document.method,
            headers: document.headers,
            body: document.body,
            retry: document.retry,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleDto {
    /// Unique identifier for schedule
    pub id: String,
    /// Optional tags to group schedules
    pub tags: Option<Tags>,
    /// Request to be executed on given schedule
    pub request: RequestDto,
    /// Schedule in cron format
    pub schedule: Option<String>,
    /// Schedule in ISO 8601 format
    pub schedule_at: Option<String>,
    /// Callback to be executed after request is executed
    pub callback: Option<CallbackDto>,
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// status
    /// - scheduled - schedule is active
    /// - executing - schedule is being executed
    /// - completed - schedule is completed
    /// - paused - schedule is paused
    /// - failed - schedule is failed
    pub status: ScheduleStatus,
}

impl From<ScheduleDocument> for ScheduleDto {
    fn from(document: ScheduleDocument) -> Self {
        Self {
            id: document.id,
            tags: document.tags,
            request: document.request.into(),
            schedule: document.schedule,
            schedule_at: document.schedule_at,
            callback: document.callback.map(|callback| callback.into()),
            created_at: document.created_at,
            updated_at: document.updated_at,
            status: document.status,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Message)]
#[rtype(result = "Result<ScheduleDto, std::io::Error>")]
pub struct CreateScheduleDto {
    /// Unique identifier for schedule
    pub id: String,
    /// Optional tags to group schedules
    pub tags: Option<Tags>,
    /// Request to be executed on given schedule
    pub request: RequestDto,
    /// Schedule in cron format
    pub schedule: Option<String>,
    /// Schedule in ISO 8601 format
    pub schedule_at: Option<String>,
    /// Callback to be executed after request is executed
    pub callback: Option<CallbackDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Message)]
#[rtype(result = "Result<ScheduleDto, std::io::Error>")]
pub struct UpdateScheduleDto {
    /// Unique identifier for schedule
    pub id: String,
    /// Optional tags to group schedules
    pub tags: Option<Vec<String>>,
    /// Request to be executed on given schedule
    pub request: RequestDto,
    /// Schedule in cron format
    pub schedule: Option<String>,
    /// Schedule in ISO 8601 format
    pub schedule_at: Option<String>,
    /// Callback to be executed after request is executed
    pub callback: Option<CallbackDto>,
}
