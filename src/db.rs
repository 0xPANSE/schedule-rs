use crate::api::dto::ScheduleDto;
use crate::db::schema::ScheduleId;
use tracing::{span, Level};

pub(crate) mod schema;

pub struct ScheduleRepository {
    schedules: sled::Tree,
}

impl ScheduleRepository {
    pub(super) fn new(db: &sled::Db) -> Self {
        Self {
            schedules: db.open_tree("schedules").unwrap(),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_schedules(
        &self,
        page: usize,
        skip: usize,
    ) -> std::io::Result<Vec<ScheduleDto>> {
        let schedules = self.schedules.clone();
        tokio::spawn(async move {
            let span = span!(Level::INFO, "schedules.get", page = %page, skip = %skip);
            let mut result: Vec<ScheduleDto> = Vec::new();
            {
                let _enter = span.enter();
                for schedule in schedules.iter().skip(skip).take(page) {
                    let schedule: ScheduleDto = serde_json::from_slice(&schedule?.1)?;
                    result.push(schedule);
                }
            }

            Ok(result)
        })
        .await?
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_schedule(&self, id: ScheduleId) -> std::io::Result<Option<ScheduleDto>> {
        let schedules = self.schedules.clone();
        tokio::spawn(async move {
            let span = span!(Level::INFO, "schedules.get", id = %id);
            let bytes = {
                let _enter = span.enter();
                schedules.get(id)?
            };
            if let Some(s) = bytes {
                let schedule: ScheduleDto = serde_json::from_slice(&s)?;
                Ok(Some(schedule))
            } else {
                Ok(None)
            }
        })
        .await?
    }
}
