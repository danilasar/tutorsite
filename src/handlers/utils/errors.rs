use actix_session::Session;
use actix_web::{get, http, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use http::header::ContentType;
use serde_json::json;
use crate::context::Context;
use crate::core::service_data::ServiceData;
use crate::core::templator;

pub async fn page_500(service_data: &ServiceData) -> actix_web::Result<HttpResponse> {
    let page = service_data.context.handlebars
        .render("errors/500", &json!({  }))
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &*page, "Я упал".into()).await;
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type(ContentType::html())
        .body(wrap))
}

pub async fn page_404(service_data: &ServiceData) -> actix_web::Result<HttpResponse> {
    let page = service_data.context.handlebars
        .render("errors/404", &json!({  }))
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &*page, "Страница не найдена".into()).await;
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type(ContentType::html())
        .body(wrap))
}

pub async fn page_403(service_data: &ServiceData) -> actix_web::Result<HttpResponse> {
    let about = service_data.context.handlebars
        .render("errors/403", &json!({ }))
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &*about, "Нет доступа".into()).await;
    return Ok(HttpResponse::build(StatusCode::FORBIDDEN)
        .content_type(ContentType::html())
        .body(wrap));
}