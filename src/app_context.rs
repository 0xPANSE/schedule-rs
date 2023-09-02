use crate::config::db::SledConfigExt;
use crate::db::ScheduleRepository;
use sled::{Db, Tree};
use tracing::{event, span, Level};

pub struct ApiContext {
    #[allow(dead_code)]
    pub db: Db,
    pub schedules: ScheduleRepository,
    #[allow(dead_code)]
    pub triggers: Tree,
}

impl ApiContext {
    pub fn new() -> Self {
        log::info!("Opening database");
        event!(Level::INFO, "Opening database");
        let db: Db = sled::Config::from_env().open().unwrap();
        if db.was_recovered() {
            event!(Level::INFO, "Database was recovered");
        } else {
            event!(Level::INFO, "Database created");
        }
        let schedules = ScheduleRepository::new(&db);
        let triggers = db.open_tree("triggers").unwrap();

        Self {
            db,
            schedules,
            triggers,
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn shutdown(&self) -> std::io::Result<()> {
        let span = span!(Level::INFO, "schedules.shutdown");
        let _enter = span.enter();
        event!(Level::INFO, "Closing database");
        self.db.flush().unwrap();
        Ok(())
    }
}

impl Drop for ApiContext {
    fn drop(&mut self) {
        self.shutdown().unwrap();
        // Ensure all spans have been shipped to Jaeger.
        opentelemetry::global::shutdown_tracer_provider();
    }
}
