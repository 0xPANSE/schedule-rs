use crate::api::dto::{CreateScheduleDto, ScheduleDto};
use crate::app_context::ApiContext;
use actix_web::error::ErrorNotFound;
use actix_web::{get, post, web, Responder};
use serde::Deserialize;
use std::sync::Arc;

pub(crate) fn endpoints() -> actix_web::Scope {
    web::scope("/api")
        .service(index)
        .service(get_schedule)
        .service(create_schedule)
}

#[derive(Clone, Deserialize)]
pub struct ListSchedulesQueryDto {
    pub page: Option<usize>,
    pub after: Option<usize>,
}

#[get("/schedules")]
pub async fn index(
    ctx: web::Data<Arc<ApiContext>>,
    query: web::Query<ListSchedulesQueryDto>,
) -> actix_web::Result<impl Responder> {
    let page = query.page.unwrap_or(50);
    let after = query.after.unwrap_or(0);
    let response: Vec<ScheduleDto> = ctx.schedules.list(page, after).await?;
    Ok(web::Json(response))
}

#[derive(Clone, Deserialize)]
pub struct GetScheduleQueryDto {
    pub id: String,
}

#[get("/schedules/{id}")]
pub async fn get_schedule(
    ctx: web::Data<Arc<ApiContext>>,
    req: web::Path<GetScheduleQueryDto>,
) -> actix_web::Result<impl Responder> {
    let response = ctx.schedules.get::<ScheduleDto>(req.id.clone()).await?;
    match response {
        Some(schedule) => Ok(web::Json(schedule)),
        None => Err(ErrorNotFound("Schedule not found")),
    }
}

#[post("/schedules")]
pub async fn create_schedule(
    ctx: web::Data<Arc<ApiContext>>,
    req: web::Json<CreateScheduleDto>,
) -> actix_web::Result<impl Responder> {
    let response = ctx.schedules.create_schedule(req.into_inner()).await?;
    Ok(web::Json(response))
}
