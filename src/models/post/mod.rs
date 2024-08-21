use crate::core::models::DbError;

#[derive(Debug)]
pub struct Post {
    id: Option<i32>,
    title: Option<String>,
    content: Option<String>
}

impl Post {
    pub async fn get_posts(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Post>, DbError> {
        Ok(sqlx::query_as::<_, Post>(include_str!("sql/get_all.sql"))
            .fetch_all(&pool) // -> Vec<Country>
            .await?)
    }
    pub async fn get_post(pool: &sqlx::Pool<sqlx::Postgres>, id:i32) -> Result<Post, DbError> {
        Ok(sqlx::query_as::<_, Post>(include_str!("sql/get_by_id.sql"))
            .bind(id)
            .execute(&pool)
            .await?
            .fetch_one());
    }
    pub async fn create(&mut self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {
        let result: (i32,) = sqlx::query(include_str!("sql/create.sql"))
            .bind(self.title.clone().unwrap_or_default())
            .bind(self.content.clone().unwrap_or_default())
            .execute(&pool)
            .await?
            .fetch_one();
        self.id = result.0;
        Ok(())
    }
    pub async fn remove_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id:i32) -> Result<(), DbError> {
        Ok(sqlx::query(include_str!("sql/delete.sql"))
            .bind(id)
            .execute(&pool)
            .await?)
    }
    pub async fn delete(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {
        self.remove_by_id(self.id);
    }
    pub async fn update(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {
        Ok(sqlx::query(include_str!("./sql/update.sql"))
            .bind(self.id.clone().unwrap_or_default())
            .bind(self.title.clone().unwrap_or_default())
            .bind(self.content.clone().unwrap_or_default())
            .execute(&pool)
            .await?)
    }
}