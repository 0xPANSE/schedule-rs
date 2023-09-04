use crate::api::schedule;
use crate::config::web::HttpServerExt;
use crate::metrics::init_telemetry;
use actix_web::dev::Service;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpMessage, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::sync::Arc;
use tracing_actix_web::{RequestId, TracingLogger};

mod api;
pub(crate) mod app_context;
// mod cluster;
pub(crate) mod config;
mod db;
mod metrics;
mod scheduler;

const HTML_404: &str = include_str!("404.html");
async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body(HTML_404)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    init_telemetry("schedule-rs");
    let ctx = Arc::new(app_context::ApiContext::new());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .wrap(Logger::default())
            .wrap_fn(|req, srv| {
                let request_id = req.extensions().get::<RequestId>().copied();
                let res = srv.call(req);
                async move {
                    let mut res = res.await?;
                    if let Some(request_id) = request_id {
                        res.headers_mut().insert(
                            HeaderName::from_static("x-request-id"),
                            HeaderValue::from_str(&request_id.to_string()).unwrap(),
                        );
                    }
                    Ok(res)
                }
            })
            .wrap(TracingLogger::default())
            .service(schedule::endpoints())
            .default_service(actix_web::web::route().to(not_found))
    })
    .set_binding_from_env()?
    .set_workers_from_env()
    .set_hostname_from_env()
    .shutdown_timeout(30)
    .run()
    .await?;

    Ok(())
}
