use crate::app_context::ApiContext;
use actix_web::error::ErrorNotFound;
use actix_web::{get, web, Responder};
use serde::Deserialize;

pub(crate) fn endpoints() -> actix_web::Scope {
    web::scope("/api").service(index).service(get_schedule)
}

#[derive(Clone, Deserialize)]
pub struct SchedulesRequest {
    pub page: Option<usize>,
    pub after: Option<usize>,
}

#[get("/schedules")]
pub async fn index(
    ctx: web::Data<ApiContext>,
    query: web::Query<SchedulesRequest>,
) -> actix_web::Result<impl Responder> {
    let page = query.page.unwrap_or(50);
    let after = query.after.unwrap_or(0);
    let response = ctx.schedules.get_schedules(page, after).await?;
    Ok(web::Json(response))
}

#[derive(Clone, Deserialize)]
pub struct GetScheduleRequest {
    pub id: String,
}

#[get("/schedules/{id}")]
pub async fn get_schedule(
    ctx: web::Data<ApiContext>,
    req: web::Path<GetScheduleRequest>,
) -> actix_web::Result<impl Responder> {
    let response = ctx.schedules.get_schedule(req.id.clone()).await?;
    match response {
        Some(schedule) => Ok(web::Json(schedule)),
        None => Err(ErrorNotFound("Schedule not found")),
    }
}
