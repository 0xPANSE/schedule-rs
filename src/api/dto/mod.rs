mod schedule;

pub use schedule::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TriggerDto {
    /// Trigger id
    pub id: String,
    /// Schedule id
    pub schedule_id: u64,
    /// Schedule at
    pub at: chrono::DateTime<chrono::Utc>,
    /// Node id
    pub node_id: u64,
}
