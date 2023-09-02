use crate::api::dto::{CreateScheduleDto, ScheduleDto, UpdateScheduleDto};
use crate::schema::{RequestHeaders, ScheduleId};
use async_raft::{AppData, AppDataResponse};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize)]
pub struct Request {
    url: String,
    method: String,
    headers: Option<RequestHeaders>,
    body: Vec<u8>,
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("url", &self.url)
            .field("method", &self.method)
            .finish()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScheduleData {
    Create(CreateScheduleDto, DateTime<chrono::Utc>),
    Update(UpdateScheduleDto, DateTime<chrono::Utc>),
    Delete(ScheduleId, DateTime<chrono::Utc>),
}

impl AppData for ScheduleData {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScheduleEventResponse {
    Created(ScheduleDto),
    Updated(ScheduleDto),
    Deleted(ScheduleId),
}

impl AppDataResponse for ScheduleEventResponse {}
