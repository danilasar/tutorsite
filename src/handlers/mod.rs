mod index;
mod post;
mod errors;
mod login;
mod utils;
mod update;

use actix_session::Session;
use actix_web::{get, HttpRequest, HttpResponse, web};
use crate::context::Context;
use crate::core::service_data::ServiceData;

async fn page_404(req: HttpRequest, context: web::Data<Context>, session: Session) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    return utils::errors::page_404(&service_data).await;
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(index::page_index);
    cfg.service(post::page_post);
    cfg.service(login::get_login);
    cfg.service(login::post_login);
    cfg.service(login::get_logout);
    cfg.service(update::update);
    cfg.default_service(
        web::route().to(page_404)
    );

}
