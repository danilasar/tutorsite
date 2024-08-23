/*
 Этот модуль отвечает за обработчики запросов. Их задача -- принять и
 валидировать данные, которые приносит питон, а далее -- сообщить об
 ошибке или передать их сервису.
 */
mod index;
mod post;
mod errors;
mod login;
mod utils;

use actix_web::web;


pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(index::page_index);
    cfg.service(post::page_post);
    cfg.service(login::get_login);
    cfg.service(login::post_login);
    cfg.service(login::get_logout);
}
