use std::sync::Arc;
use handlebars::Handlebars;

#[derive(Clone, Debug)]
pub struct  Context {
    pub db : sqlx::Pool<sqlx::Postgres>,
    pub handlebars: Arc<Handlebars<'static>>
}