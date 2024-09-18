use actix_session::Session;
use actix_web::{get, post, http, HttpRequest, HttpResponse, web};
use actix_web::http::{header, StatusCode};
use http::header::ContentType;

use crate::context::Context;
use crate::core::service_data::ServiceData;
use crate::handlers::utils;
use crate::services;

#[get("/update")]
async fn update(req: HttpRequest,
                    context: web::Data<Context>,
                    session: Session
) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    if(!services::users::is_authored(&service_data).await) {
        return utils::errors::page_403(&service_data).await;
    }
    log::info!("Запущено обновление данных");
    if let Err(e) = services::git::sync_posts(&service_data).await {
        log::info!("{:#?}", e);
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body("aboba"))
}