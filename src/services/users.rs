use std::hash::{BuildHasher, Hasher};
use actix_session::{Session, SessionGetError};
use rs_sha512::{HasherContext, Sha512State};
use serde::{Deserialize, Serialize};
use crate::core::models::DbError;
use crate::core::service_data::ServiceData;
use crate::models::user::User;

pub enum GetCurrentUserError {
    SessionGet(SessionGetError), Db(DbError), SessionIsNotString
}

#[derive(PartialEq)]
pub enum AuthError {
    BadName,
    BadLogin,
    BadPassword,
    AlreadyExists,
    NotFound,
    TokenNotGenerated,
    CookieNotWrote
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

pub(crate) fn hash_password(password:&str, login: &str) -> String {
    let mut sha512hasher = Sha512State::default().build_hasher();
    sha512hasher.write(password.as_bytes());
    sha512hasher.write(format!("СВО{}aboba_AntiHohol",
                               login.clone()).as_bytes());
    let bytes_result = HasherContext::finish(&mut sha512hasher);
    return format!("{bytes_result:02x}");
}

pub async fn get_current_token(service_data: &ServiceData) -> Result<String, GetCurrentUserError> {
    match service_data.session.get("token") {
        Ok(token_option) => match token_option {
            Some(val) => Ok(val),
            None => Err(GetCurrentUserError::SessionIsNotString)
        },
        Err(error) => Err(GetCurrentUserError::SessionGet(error))
    }
}

pub async fn get_current_user(service_data: &ServiceData)
                              -> Result<User, GetCurrentUserError>
{
    let token : String = get_current_token(service_data).await?;

    match User::get_by_token(&service_data.context.db, token.as_str()).await {
        Ok(user) => Ok(user),
        Err(error) => Err(GetCurrentUserError::Db(error))
    }
}

pub async fn is_authored(service_data: &ServiceData) -> bool {
    return get_current_user(service_data).await.is_ok();
}