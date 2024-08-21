use actix_session::Session;
use actix_web::{get, http, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use http::header::ContentType;
use serde_json::json;
use crate::context::Context;
use crate::core::service_data::ServiceData;
use crate::core::templator;

#[get("/")]
async fn page_index(req: HttpRequest, context: web::Data<Context>, session: Session) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await?;
    let posts = crate::services::posts::get_posts(&service_data.context);
    let about = context.handlebars
        .render("index", &json!({ posts: posts }))
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &*about, "О доме".into()).await;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(wrap))
}