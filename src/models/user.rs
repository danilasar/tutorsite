use serde::{Deserialize, Serialize};
use crate::core::models::DbError;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub password_hash: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserRegisterForm {
    pub login: String,
    pub name: String,
    pub password: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserLoginForm {
    pub login: String,
    pub password: String
}

impl User {

    pub async fn add_user(pool: &sqlx::Pool<sqlx::Postgres>, mut user_info: User) -> Result<User, DbError> {
        if(user_info.login.is_none() || user_info.password_hash.is_none()) {
            return Err(DbError::InvalidData);
        }
        let login = user_info.login.clone().unwrap();
        let result = sqlx::query_file!("sql/user/add.sql",
            login.clone(),
            user_info.name.clone().unwrap_or(login),
            user_info.password_hash.clone().unwrap()).fetch_one(pool).await;
        user_info.id = Option::from(result.unwrap().id);
        Ok(user_info)
    }

    pub async fn get_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id:i32) -> Result<User, DbError>
    {
        let result = sqlx::query_file_as!(User, "sql/user/get_by_id.sql", id).fetch_one(pool).await;
        match result {
            Ok(u) => Ok(u),
            Err(e) => Err(DbError::InternalError(e))
        }
    }

    pub async fn get_by_login(pool: &sqlx::Pool<sqlx::Postgres>, login:&str) -> Result<User, DbError> {
        let result = sqlx::query_file_as!(User, "sql/user/get_by_login.sql", login).fetch_one(pool).await;
        match result {
            Ok(u) => Ok(u),
            Err(e) => Err(DbError::InternalError(e))
        }
    }

    pub async fn get_by_token(pool: &sqlx::Pool<sqlx::Postgres>, token: &str) -> Result<User, DbError> {
        let result = sqlx::query_file_as!(User, "sql/user/get_by_token.sql", token).fetch_one(pool).await;
        match result {
            Ok(u) => Ok(u),
            Err(e) => Err(DbError::InternalError(e))
        }
    }
}