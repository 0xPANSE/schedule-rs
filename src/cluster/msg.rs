use async_raft::{AppData, AppDataResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScheduleEvent {
    Scheduled {
        id: String,
        name: String,
        at: String,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    },
    ScheduleUpdated {
        id: String,
        name: String,
        cron: String,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    },
    ScheduleDeleted {
        id: String,
    },
    ScheduleTriggered {
        id: String,
        name: String,
        cron: String,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    },
    ScheduleTriggeredError {
        id: String,
        name: String,
        cron: String,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    },
    ScheduleTriggeredSuccess {
        id: String,
        name: String,
        cron: String,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    },
}

impl AppData for ScheduleEvent {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScheduleEventResponse {
    Ok { id: String },
    Error { id: String, error: String },
}

impl AppDataResponse for ScheduleEventResponse {}
