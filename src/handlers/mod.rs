/*
 Этот модуль отвечает за обработчики запросов. Их задача -- принять и
 валидировать данные, которые приносит питон, а далее -- сообщить об
 ошибке или передать их сервису.
 */
mod index;
mod post;
mod errors;

use actix_web::web;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(index::page_index);
}
