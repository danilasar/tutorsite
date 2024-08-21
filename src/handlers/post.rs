use actix_session::Session;
use actix_web::{get, http, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use http::header::ContentType;
use serde_json::json;
use crate::context::Context;
use crate::core::service_data::ServiceData;
use crate::core::templator;
use crate::handlers::errors;

#[get("/post/{id}")]
async fn page_post(req: HttpRequest, context: web::Data<Context>, session: Session, path: web::Path<(u32,)>) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    let post = crate::services::posts::get_post(&service_data.context, path.into_inner().0 as i32).await;

    if post.is_none() {
        return errors::page_404(&service_data).await;
    }

    let about = service_data.context.handlebars
        .render("index", &json!({ "posts": post.clone().unwrap() }))
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &*about, Option::from(post.unwrap().title.unwrap_or_default().as_str())).await;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(wrap))
}