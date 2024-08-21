use actix_web::{get, http, HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use http::header::ContentType;

#[get("/posts")]
async fn get_posts(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body("Aboba"))
}