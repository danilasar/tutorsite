use actix_session::Session;
use actix_web::{get, http, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use http::header::ContentType;
use serde_json::json;
use crate::context::Context;
use crate::core::service_data::ServiceData;
use crate::core::templator;
use crate::services;

#[get("/")]
async fn page_index(req: HttpRequest, context: web::Data<Context>, session: Session) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    /*models::user::User::add_user(&service_data.context.db, models::user::User {
        id: None,
        login: Option::from("логин".to_string()),
        name: Option::from("имя".to_string()),
        password_hash: Option::from(services::users::hash_password("пароль", "логин"))
    }).await;*/
    let posts = services::posts::get_posts_list(&service_data.context).await;
    let about = service_data.context.handlebars
        .render("index", &json!({
            "posts": posts,
            "authored": services::users::is_authored(&service_data).await,
        }))
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &*about, "Главная".into()).await;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(wrap))
}