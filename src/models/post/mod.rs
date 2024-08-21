use crate::core::models::DbError;
pub struct Post {
    id: Option<i32>,
    title: Option<String>,
    content: Option<String>
}

impl Post {
    fn get_posts(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Post>, DbError> {
        
    }
    fn add(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {

    }
    fn remove_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id:i32) -> Result<(), DbError> {

    }
    fn delete(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {
        self.remove_by_id(self.id);
    }
    fn update(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {

    }
}