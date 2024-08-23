use std::hash::{BuildHasher, Hasher};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use rs_sha512::{HasherContext, Sha512State};
use serde::{Deserialize, Serialize};
use crate::core::models::DbError;
use crate::models::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub key: Option<String>,
    pub user_id: Option<i32>,
    pub expires: Option<DateTime<Utc>> // utc timestamp
}

impl Session {
    pub async fn remove_by_token(pool: &sqlx::Pool<sqlx::Postgres>, token: &str) -> Result<(), DbError> {
        match sqlx::query_file!("sql/session/remove_by_token.sql", token)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DbError::InternalError(e))
        }
    }

    pub async fn remove_user_sessions(pool: &sqlx::Pool<sqlx::Postgres>, user: User) -> Result<(), DbError> {
        match sqlx::query_file!("sql/session/remove_by_user.sql", user.id)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DbError::InternalError(e))
        }
    }


    pub async fn generate_token(pool: &sqlx::Pool<sqlx::Postgres>,
                                        mut user: User,
                                        lifetime: Option<Duration>)
                                        -> Result<Session, (DbError)>
    {
        if (user.id.is_none()) {
            if (user.login.is_none()) {
                return Err(DbError::NotFound);
            }
            match User::get_by_login(pool,
                                           user.login.unwrap().as_str()).await
            {
                Ok(usr) => user.id = usr.id,
                Err(e) => return Err(e)
            }
        }
        let mut rng = rand::thread_rng();
        let seed = rng.gen::<u128>();
        let mut sha512hasher = Sha512State::default().build_hasher();
        sha512hasher.write(seed.to_string().as_bytes());
        let bytes_result = HasherContext::finish(&mut sha512hasher);
        let token = format!("{bytes_result:02x}");

        let mut datetime = DateTime::default();

        if (lifetime.is_none()) {
            let result = sqlx::query_file!(
                "sql/session/add.sql",
                token,
                user.id
            )
                .fetch_one(pool).await;
            if let Err(e) = result {
                return Err(DbError::InternalError(e));
            }
            datetime = result.unwrap().expires.and_utc();
        } else {
            datetime = Utc::now() + lifetime.unwrap();
            let result = sqlx::query_file!(
                "sql/session/add_expirable.sql",
                token,
                user.id,
                datetime.naive_utc()
            )
                .execute(pool).await;
            if let Err(e) = result {
                return Err(DbError::InternalError(e));
            }
        }
        Ok(Session {
            key: Option::from(token),
            user_id: user.id,
            expires: Option::from(datetime),
        })
    }
}