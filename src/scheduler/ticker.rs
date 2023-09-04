use crate::db::schema::ScheduleDocument;
use std::str::FromStr;

/// Holds the information about when the next job should be run.
pub enum Ticker {
    ScheduleAt(chrono::DateTime<chrono::Utc>),
    Cron(cron::Schedule),
}

impl Ticker {
    /// Returns the next time the job should be run. Or None if the job should not be run anymore.
    pub fn next(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self {
            Ticker::ScheduleAt(dt) => Some(*dt),
            Ticker::Cron(cron) => cron.upcoming(chrono::Utc).next(),
        }
    }

    /// Returns the next time the job should be run after the given date.
    /// Or None if the job should not be run anymore.
    pub fn next_after(
        &self,
        date: &chrono::DateTime<chrono::Utc>,
    ) -> Option<chrono::DateTime<chrono::Utc>> {
        match self {
            Ticker::ScheduleAt(dt) => {
                if dt > date {
                    Some(*dt)
                } else {
                    None
                }
            }
            Ticker::Cron(cron) => cron.after(date).next(),
        }
    }
}

impl TryFrom<ScheduleDocument> for Ticker {
    /// Creates a Ticker from a ScheduleDocument
    type Error = String;

    fn try_from(value: ScheduleDocument) -> Result<Self, Self::Error> {
        if let Some(dt) = value.schedule_at {
            Ok(Self::ScheduleAt(dt.parse().map_err(|_| {
                format!("schedule_at format is not ISO8601: {}", dt)
            })?))
        } else if let Some(c) = value.schedule {
            Ok(Self::Cron(cron::Schedule::from_str(c.as_str()).map_err(
                |_| format!("schedule format is not cron: {}", c),
            )?))
        } else {
            Err(format!("ScheduleDocument has no schedule or schedule_at"))
        }
    }
}
