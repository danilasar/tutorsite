use crate::core::models::DbError;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>
}

impl Post {
    pub async fn get_posts(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Post>, DbError> {
        let posts = sqlx::query_file_as!(Post, "sql/post/get_all.sql")
            .fetch_all(pool) // -> Vec<Country>
            .await;
        if let Err(e) = posts {
            return Err(DbError::InternalError(e));
        }
        Ok(posts.unwrap())
    }
    pub async fn get_posts_list(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Post>, DbError> {
        let posts = sqlx::query_file!("sql/post/get_list.sql")
            .fetch_all(pool) // -> Vec<Country>
            .await;
        if let Err(e) = posts {
            return Err(DbError::InternalError(e));
        }
        let posts = posts.unwrap().iter().map(|p| {
            Post {
                id: Option::from(p.id),
                title: Option::from(p.title.clone()),
                description: Option::from(p.description.clone()),
                content: None
            }
        }).collect::<Vec<Post>>();

        Ok(posts)
    }
    pub async fn get_post(pool: &sqlx::Pool<sqlx::Postgres>, id:i32) -> Result<Post, DbError> {
        let post = sqlx::query_file_as!(Post, "sql/post/get_by_id.sql", id)
            .fetch_one(pool)
            .await;
        if let Err(e) = post {
            return Err(DbError::InternalError(e));
        }
        Ok(post.unwrap())
    }
    pub async fn create(&mut self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {
        let q = sqlx::query_file!(
                "sql/post/create.sql",
                self.title.clone().unwrap_or_default(),
                self.description.clone().unwrap_or_default(),
                self.content.clone().unwrap_or_default())
            .fetch_one(pool)
            .await;
        if let Err(e) = q {
            return Err(DbError::InternalError(e));
        }
        self.id = Option::from(q.unwrap().id);
        Ok(())
    }
    pub async fn remove_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id:i32) -> Result<(), DbError> {
        match sqlx::query_file!("sql/post/delete.sql", id)
            .execute(pool)
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(DbError::InternalError(e))
        }
    }
    pub async fn delete(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {
        if self.id.is_none() {
            return Err(DbError::InvalidData)
        }
        Self::remove_by_id(pool, self.id.unwrap()).await?;
        Ok(())
    }
    pub async fn update(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), DbError> {
        if let Err(e) = sqlx::query_file!("sql/post/update.sql",
            self.id.clone().unwrap_or_default(),
            self.title.clone().unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
            self.content.clone().unwrap_or_default()
        )
            .execute(pool)
            .await
        {
            return Err(DbError::InternalError(e));
        }
        Ok(())
    }
}