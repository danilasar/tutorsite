
#[derive(Clone, Debug)]
pub struct  Context {
    pub db : sqlx::Pool<sqlx::Postgres>
}