/*
 Этот модуль отвечает за обработчики запросов. Их задача -- принять и
 валидировать данные, которые приносит питон, а далее -- сообщить об
 ошибке или передать их сервису.
 */
mod posts;

use actix_web::web;
use crate::handlers::posts::index::{page_index};

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(page_index);
}