use std::env;
use std::sync::Arc;
use actix_session::config::CookieContentSecurity;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::{App, HttpServer};
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::web::Data;
use figment::Figment;
use figment::providers::Env;
use handlebars::{DirectorySourceOptions, Handlebars};
use crate::context::Context;

mod context;
mod config;
mod handlers;
mod services;
mod models;
mod core;

use sqlx;

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(), Key::from(&[0; 64])
    )
        .cookie_secure(false) // https и http
        //.session_lifecycle(BrowserSession::default()) // expire at end of session
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private) // encrypt
        .cookie_http_only(false) // не отключать чтение скриптами
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    _ = dotenvy::dotenv();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let config: config::Config = Figment::new()
        .merge(Env::raw())
        .extract()
        .unwrap_or_else(|e| {
            log::error!("Ошибка создания конфига: \n{:#?}", e);
            panic!("Не могу создать конфиг");
        });

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&*config.postgres_url).await.unwrap_or_else(|e| {
            log::error!("Ошибка подключения к базе: \n{:#?}", e);
            panic!("Не могу подключиться к базе");
        });

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory("views", DirectorySourceOptions::default())
        .unwrap();

    let ctx = Context {
        db: pool,
        handlebars: Arc::new(handlebars)
    };

    log::info!("Сервер хостится на http://{}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ctx.clone()))
            .wrap(session_middleware())
            .configure(handlers::setup)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
    })
        .bind((config.host, config.port))
        .unwrap_or_else(|e| {
            log::error!("Ошибка роутинга: \n{:#?}", e);
            panic!("Ошибка роутинга");
        })
        .run()
        .await
}
