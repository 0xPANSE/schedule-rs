use std::sync::Arc;

use actix::{
    Actor, ActorContext, ActorFutureExt, AsyncContext, Context, Handler, Message, SpawnHandle,
};

use crate::db::schema::{ScheduleDocument, ScheduleId};
use crate::db::ScheduleRepository;
use crate::scheduler::ticker::Ticker;

pub struct ScheduleActor {
    id: ScheduleId,
    state: Option<ScheduleDocument>,
    ticker: Option<Ticker>,
    last_tick: Option<chrono::DateTime<chrono::Utc>>,
    next_tick: Option<chrono::DateTime<chrono::Utc>>,
    repo: Arc<ScheduleRepository>,
    cancel_hnd: Option<SpawnHandle>,
}

impl ScheduleActor {
    pub fn new(id: ScheduleId, repo: Arc<ScheduleRepository>) -> Self {
        Self {
            id,
            state: None,
            ticker: None,
            last_tick: None,
            next_tick: None,
            repo,
            cancel_hnd: None,
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Tick(chrono::DateTime<chrono::Utc>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct StopIfNoSchedule;

impl Actor for ScheduleActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Starting {}", self.id);
        let repo = self.repo.clone();
        let id = self.id.clone();
        let f = async move { repo.get::<ScheduleDocument>(id).await };
        let w = actix::fut::wrap_future::<_, Self>(f).map(|res, act, ctx| match res {
            Ok(Some(ref schedule)) => {
                log::info!("Found schedule for {}", act.id);
                let t: Result<Ticker, String> = schedule.to_owned().try_into();
                if let Ok(ticker) = t {
                    let after = schedule.last_run.unwrap_or(chrono::Utc::now());
                    let next_tick = ticker.next_after(&after);
                    if next_tick.is_none() {
                        log::debug!("No next tick for {}, stopping", act.id);
                        ctx.stop();
                        return;
                    }
                    let next_tick = next_tick.unwrap();
                    act.state = Some(schedule.clone());
                    act.ticker = Some(ticker);
                    act.last_tick = schedule.last_run;
                    act.next_tick = Some(next_tick);
                    let timeout = next_tick.signed_duration_since(chrono::Utc::now());
                    if timeout <= chrono::Duration::zero() {
                        log::debug!("Next tick for {} is in the past, running now", act.id);
                        ctx.notify(Tick(next_tick));
                    } else {
                        log::debug!("Next tick for {} in {}", act.id, timeout);
                        let cancel_hnd =
                            ctx.notify_later(Tick(next_tick), timeout.to_std().unwrap());
                        act.cancel_hnd = Some(cancel_hnd);
                    }
                } else {
                    log::error!("Error while parsing schedule for {}. Stopping", act.id);
                    // restart actor
                    ctx.stop();
                }
            }
            Ok(None) => {
                // Can't find schedule, we should wait for CreateSchedule message
                log::info!("Creating {}", act.id);
                ctx.notify_later(StopIfNoSchedule, std::time::Duration::from_secs(30));
            }
            Err(e) => {
                log::error!("error getting schedule for {}: {}", act.id, e);
                ctx.stop();
            }
        });
        ctx.wait(w);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::debug!("stopped for {}", self.id);
    }
}

impl Handler<Tick> for ScheduleActor {
    type Result = ();

    fn handle(&mut self, msg: Tick, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<StopIfNoSchedule> for ScheduleActor {
    type Result = ();

    fn handle(&mut self, _msg: StopIfNoSchedule, ctx: &mut Self::Context) -> Self::Result {
        if self.state.is_none() {
            log::debug!(
                "Nothing scheduled in past 30 seconds for {}, stopping",
                self.id
            );
            ctx.stop();
        }
    }
}
