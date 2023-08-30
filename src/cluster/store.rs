use super::msg::{ScheduleEvent, ScheduleEventResponse};
use crate::cluster::snapshot::ClusterSnapshot;
use anyhow::{Context, Result};
use async_raft::raft::{Entry, EntryPayload, MembershipConfig};
use async_raft::storage::{CurrentSnapshotData, HardState, InitialState};
use async_raft::{NodeId, RaftStorage};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sled::transaction::{ConflictableTransactionError, TransactionError};
use sled::Batch;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterState {
    pub last_applied_log: u64,
}

// Sled backed storage for raft cluster
pub struct SchedulerRaftStorage {
    // node id
    id: NodeId,
    // sled db
    db: sled::Db,
    // current cluster state
    state: RwLock<ClusterState>,
    // current HardState
    hard_state: RwLock<Option<HardState>>,
    // current snapshot
    current_snapshot: RwLock<Option<CurrentSnapshotData<ClusterSnapshot>>>,
}

impl SchedulerRaftStorage {
    pub fn new(id: NodeId) -> Self {
        let db = sled::open("schedule-rs.mdb").unwrap();
        let state = RwLock::new(ClusterState::default());
        let hard_state = RwLock::new(None);
        let current_snapshot = RwLock::new(None);
        Self {
            id,
            db,
            state,
            hard_state,
            current_snapshot,
        }
    }
}

#[derive(Debug, Error)]
#[error("Sled error: {0}")]
pub struct ShutdownError(TransactionError<anyhow::Error>);

#[async_trait]
impl RaftStorage<ScheduleEvent, ScheduleEventResponse> for SchedulerRaftStorage {
    type Snapshot = ClusterSnapshot;
    type ShutdownError = ShutdownError;

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_membership_config(&self) -> Result<MembershipConfig> {
        let mut current_key = u64::MAX.to_be_bytes().to_vec();
        while let Some((key_vec, value_vec)) = self.db.get_lt(current_key.clone())? {
            let value = serde_json::from_slice::<Entry<ScheduleEvent>>(value_vec.as_ref())?;
            let cfg_opt = match value.payload {
                EntryPayload::ConfigChange(cfg) => Some(cfg.membership.clone()),
                EntryPayload::SnapshotPointer(snap) => Some(snap.membership.clone()),
                _ => None,
            };
            if let Some(cfg) = cfg_opt {
                return Ok(cfg);
            }
            current_key = key_vec.to_vec();
        }
        Ok(MembershipConfig::new_initial(self.id))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_initial_state(&self) -> Result<InitialState> {
        let membership = self.get_membership_config().await?;
        let mut hs = self.hard_state.write().await;
        let log_entry = self.db.last()?;
        let sm = self.state.read().await;
        match &mut *hs {
            Some(inner) => {
                let (last_log_index, last_log_term) = match log_entry {
                    Some((_, value_vec)) => {
                        let value =
                            serde_json::from_slice::<Entry<ScheduleEvent>>(value_vec.as_ref())?;
                        (value.index, value.term)
                    }
                    None => (0, 0),
                };
                let last_applied_log = sm.last_applied_log;
                Ok(InitialState {
                    last_log_index,
                    last_log_term,
                    last_applied_log,
                    hard_state: inner.clone(),
                    membership,
                })
            }
            None => {
                let new = InitialState::new_initial(self.id);
                *hs = Some(new.hard_state.clone());
                Ok(new)
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self, hs))]
    async fn save_hard_state(&self, hs: &HardState) -> Result<()> {
        *self.hard_state.write().await = Some(hs.clone());
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_log_entries(&self, start: u64, stop: u64) -> Result<Vec<Entry<ScheduleEvent>>> {
        // Invalid request, return empty vec.
        if start > stop {
            tracing::error!("invalid request, start > stop");
            return Ok(vec![]);
        }
        let start = start.to_be_bytes().to_vec();
        let stop = stop.to_be_bytes().to_vec();
        let mut entries = vec![];
        for value_vec in self.db.range(start..stop).values() {
            let value = serde_json::from_slice::<Entry<ScheduleEvent>>(value_vec?.as_ref())?;
            entries.push(value);
        }

        Ok(entries)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn delete_logs_from(&self, start: u64, stop: Option<u64>) -> Result<()> {
        if stop.as_ref().map(|stop| &start > stop).unwrap_or(false) {
            tracing::error!("invalid request, start > stop");
            return Ok(());
        }

        // If a stop point was specified, delete from start until the given stop point.
        if let Some(stop) = stop.as_ref() {
            // remove all keys in range [start, stop) in batch
            let mut batch = Batch::default();
            for key in start..*stop {
                batch.remove(key.to_be_bytes().to_vec());
            }
            self.db
                .transaction(|db| {
                    db.apply_batch(&batch)?;
                    // todo: is it necessary to flush here?
                    // db.flush();
                    Ok(())
                })
                .map_err(|e: TransactionError<anyhow::Error>| {
                    anyhow::anyhow!("sled error: {}", e)
                })?;
            return Ok(());
        }
        // Else, just split off the remainder.
        let mut current_key = start.to_be_bytes().to_vec();
        while let Some((key_vec, _)) = self.db.get_gt(current_key.clone())? {
            self.db.remove(key_vec.to_vec())?;
            current_key = key_vec.to_vec();
        }
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self, entry))]
    async fn append_entry_to_log(&self, entry: &Entry<ScheduleEvent>) -> Result<()> {
        let key = entry.index.to_be_bytes().to_vec();
        let value = serde_json::to_vec(entry)?;
        self.db
            .transaction(|db| {
                if let None = db.insert(key.as_slice(), value.as_slice())? {
                    return Err(ConflictableTransactionError::Abort(anyhow::anyhow!(
                        "Key {} already exists.",
                        entry.index
                    )));
                }
                db.flush();
                Ok(())
            })
            .map_err(|e: TransactionError<anyhow::Error>| ShutdownError(e))
            .context("Raft append_entry_to_log error")
    }

    #[tracing::instrument(level = "trace", skip(self, entries))]
    async fn replicate_to_log(&self, entries: &[Entry<ScheduleEvent>]) -> Result<()> {
        // serialize entries
        let mut batch: Batch = Batch::default();
        for entry in entries {
            batch.insert(
                entry.index.to_be_bytes().to_vec(),
                serde_json::to_vec(entry)?,
            );
        }
        // insert entries as batch in transaction
        self.db
            .transaction(|db| {
                db.apply_batch(&batch)?;
                db.flush();
                Ok(())
            })
            .map_err(|e: TransactionError<anyhow::Error>| anyhow::anyhow!("sled error: {}", e))
    }

    #[tracing::instrument(level = "trace", skip(self, data))]
    async fn apply_entry_to_state_machine(
        &self,
        index: &u64,
        data: &ScheduleEvent,
    ) -> Result<ScheduleEventResponse> {
        let mut sm = self.state.write().await;
        sm.last_applied_log = *index;
        todo!("apply this to state machine, store in database")
    }

    #[tracing::instrument(level = "trace", skip(self, entries))]
    async fn replicate_to_state_machine(&self, entries: &[(&u64, &ScheduleEvent)]) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn do_log_compaction(&self) -> Result<CurrentSnapshotData<Self::Snapshot>> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn create_snapshot(&self) -> Result<(String, Box<Self::Snapshot>)> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self, snapshot))]
    async fn finalize_snapshot_installation(
        &self,
        index: u64,
        term: u64,
        delete_through: Option<u64>,
        id: String,
        snapshot: Box<Self::Snapshot>,
    ) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_current_snapshot(&self) -> Result<Option<CurrentSnapshotData<Self::Snapshot>>> {
        todo!()
    }
}
