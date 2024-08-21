/*
 Этот модуль отвечает за обработчики запросов. Их задача -- принять и
 валидировать данные, которые приносит питон, а далее -- сообщить об
 ошибке или передать их сервису.
 */
mod posts;

use actix_web::web;
use crate::handlers::posts::get_posts::get_posts;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts);
}