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

    let repo = std::fs::read_dir("static/repo").unwrap(); // это гг, если репы нет
    for entry in repo {
        if(entry.is_err()) {
            continue;
        }
        let entry = entry.unwrap();
        let is_dir = match entry.file_type() {
            Ok(t) => t.is_dir(),
            Err(_) => false
        };
        if(!is_dir) {
            continue;
        }
        let fname = entry.file_name();
        let mount_path = "/".to_string() + fname.to_str().unwrap_or_default();
        let serve_from = "static/repo/".to_string() + fname.to_str().unwrap_or_default();
        cfg.service(actix_files::Files::new(mount_path.as_str(), serve_from.as_str()).show_files_listing());
    }

}
