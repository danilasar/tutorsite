use actix_session::Session;
use actix_web::{get, post, http, HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use http::header::ContentType;
use serde_json::json;
use crate::context::Context;
use crate::core::service_data::ServiceData;
use crate::core::templator;
use crate::handlers::{errors, utils};
use crate::services;

#[get("/post/{id}")]
async fn page_post(req: HttpRequest, context: web::Data<Context>, session: Session, path: web::Path<(u32,)>) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    let post = services::posts::get_post(&service_data.context, path.into_inner().0 as i32).await;

    if post.is_none() {
        return utils::errors::page_404(&service_data).await;
    }

    let mut post = post.unwrap();

    /*let options = &markdown::Options::gfm();

    match markdown::to_html_with_options(post.content.unwrap_or_default().as_str(),
                                         &markdown::Options {
                                             compile: markdown::CompileOptions {
                                                 allow_dangerous_html: true,
                                                 allow_dangerous_protocol: true,
                                                 ..markdown::CompileOptions::default()
                                             },
                                             ..markdown::Options::default()
                                         }
    ) {
        Ok(md) => post.content = Option::from(md),
        Err(e) => return utils::errors::page_500(&service_data).await
    }*/

    let about = service_data.context.handlebars
        .render("post", &json!({
            "post": post.clone(),
            "authored": services::users::is_authored(&service_data).await
        }))
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &*about, Option::from(post.title.unwrap_or_default().as_str())).await;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(wrap))
}

#[get("/post/add")]
async fn page_add_post(req: HttpRequest, context: web::Data<Context>, session: Session) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    if(!services::users::is_authored(&service_data).await) {
        return utils::errors::page_403(&service_data).await;
    }
    let page = service_data.context.handlebars
        .render("add_post", &json!({ }))
        .unwrap_or_default();
    let wrap = templator::wrap_page(&service_data, &*page, Option::from("Новый гайд")).await;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(wrap))
}

/*
#[post("/post/add")]
async fn add_post(req: HttpRequest, context: web::Data<Context>, session: Session, params: web::Form<>) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    if(!services::users::is_authored(&service_data).await) {
        return utils::errors::page_403(&service_data).await;
    }

}
 */