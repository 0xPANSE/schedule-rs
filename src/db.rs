use crate::api::dto::{CreateScheduleDto, ScheduleDto};
use crate::db::schema::{ScheduleDocument, ScheduleId};
use sled::Tree;
use tracing::{span, Level};

pub(crate) mod schema;

pub struct ScheduleRepository {
    schedules: Tree,
}

impl ScheduleRepository {
    pub fn new(db: &sled::Db) -> Self {
        Self {
            schedules: db.open_tree("schedules").unwrap(),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn list<T>(&self, page: usize, skip: usize) -> std::io::Result<Vec<T>>
    where
        T: From<ScheduleDocument> + Send + Sync + 'static,
    {
        let schedules = self.schedules.clone();
        tokio::spawn(async move {
            let span = span!(Level::INFO, "schedules.get", page = %page, skip = %skip);
            let mut result: Vec<T> = Vec::new();
            {
                let _enter = span.enter();
                for schedule in schedules.iter().skip(skip).take(page) {
                    let schedule: ScheduleDocument = serde_json::from_slice(&schedule?.1)?;
                    result.push(schedule.into());
                }
            }

            Ok(result)
        })
        .await?
    }

    #[tracing::instrument(skip(self))]
    pub async fn get<T>(&self, id: ScheduleId) -> std::io::Result<Option<T>>
    where
        T: From<ScheduleDocument> + serde::de::DeserializeOwned + Send + Sync + 'static,
    {
        let schedules = self.schedules.clone();
        tokio::spawn(async move {
            let span = span!(Level::INFO, "schedules.get", id = %id);
            let bytes = {
                let _enter = span.enter();
                schedules.get(id)?
            };
            if let Some(s) = bytes {
                let schedule: T = serde_json::from_slice(&s)?;
                Ok(Some(schedule))
            } else {
                Ok(None)
            }
        })
        .await?
    }

    #[tracing::instrument(skip(self))]
    pub(crate) async fn create_schedule(
        &self,
        params: CreateScheduleDto,
    ) -> std::io::Result<ScheduleDto> {
        let schedules = self.schedules.clone();
        let id = params.id.clone();
        tokio::spawn(async move {
            let span = span!(Level::INFO, "schedules.create");
            let schedule = ScheduleDto {
                id: id.clone(),
                tags: params.tags,
                request: params.request,
                schedule: params.schedule,
                schedule_at: params.schedule_at,
                callback: params.callback,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                status: schema::ScheduleStatus::Scheduled,
            };
            {
                let _enter = span.enter();
                let bytes = serde_json::to_vec(&schedule)?;
                let _ = schedules.insert(id, bytes)?;
            }
            Ok(schedule)
        })
        .await?
    }
}
