use actix_session::Session;
use actix_web::{get, post, http, HttpRequest, HttpResponse, web};
use actix_web::http::{header, StatusCode};
use http::header::ContentType;
use regex::Regex;
use serde_json::json;
use crate::context::Context;
use crate::core::service_data::ServiceData;
use crate::core::templator;
use crate::handlers::utils;
use crate::services;
use crate::models;
use crate::services::users::AuthError;

fn validate_login_form(form: services::users::UserLoginForm) -> Result<(), Vec<AuthError>> {
    let regex_login: Regex = Regex::new(r"[А-Яа-яA-Za-z0-9\-_()*&^%$#@!+=/,.{}\[\]]+").unwrap();
    let mut auth_errors:Vec<AuthError> = Default::default();
    if(!regex_login.is_match(form.login.as_str())) {
        auth_errors.push(AuthError::BadLogin);
    }
    if(!regex_login.is_match(form.password.as_str())) {
        auth_errors.push(AuthError::BadPassword);
    }
    if(!auth_errors.is_empty()) {
        return Err(auth_errors);
    }
    Ok(())
}

async fn generate_login_page(service_data: &ServiceData,
                             user: Option<models::user::User>,
                             errors : &Vec<AuthError>)
                             -> HttpResponse
{
    let mut data = json!({
                "auth_errors": {
                    "login": errors.contains(&AuthError::BadLogin),
                    "password": errors.contains(&AuthError::BadPassword),
                    "not_found": errors.contains(&AuthError::NotFound),
                    "session": errors.contains(&AuthError::TokenNotGenerated),
                    "cookie": errors.contains(&AuthError::CookieNotWrote)
                }
            });
    if(user.is_some()) {
        data["user"] = json!(user.unwrap());
    }
    let login = service_data.context.handlebars
        .render("pages/login", &data)
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &login, "Вход".into()).await;
    return HttpResponse::build(if errors.is_empty() { StatusCode::OK } else { StatusCode::BAD_REQUEST })
        .content_type(ContentType::html())
        .body(wrap);
}

#[get("/login")]
async fn get_login(req: HttpRequest, context: web::Data<Context>, session: Session) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    if services::users::is_authored(&service_data).await {
        return utils::errors::page_403(&service_data).await;
    }
    let login = service_data.context.handlebars
        .render("login", &json!({ }))
        .unwrap_or_default();

    let wrap = templator::wrap_page(&service_data, &*login, "Вход".into()).await;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(wrap))
}

#[post("/login")]
async fn post_login(req: HttpRequest,
                    context: web::Data<Context>,
                    session: Session,
                    params: web::Form<services::users::UserLoginForm>) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    if(services::users::is_authored(&service_data).await) {
        return utils::errors::page_403(&service_data).await;
    }
    let validation_result = validate_login_form(params.0.clone());
    let mut found = false; // true потому что так надо
    let mut user = models::user::User {
        id: None,
        login: None,
        name: None,
        password_hash: None
    };
    if validation_result.is_ok() {
        match models::user::User::get_by_login(&service_data.context.db, params.login.as_str()).await {
            Ok(usr) => {
                user = usr;
                let hash = services::users::hash_password(params.password.as_str(), params.login.as_str());
                found = user.password_hash.clone().unwrap().as_str().eq(&hash);
            },
            Err(..) => found = false
        }
    }
    if validation_result.is_err() || !found {
        let mut errors = match validation_result {
            Ok(_) => Vec::new(),
            Err(E) => E
        };
        errors.push(AuthError::NotFound);
        return Ok(generate_login_page(&service_data, Option::from(user), &errors).await);
    }

    let session_token = models::session::Session::generate(
        &service_data.context.db,
        user.clone(), None
    ).await;

    if(session_token.is_err()) {
        return Ok(generate_login_page(&service_data,
            Option::from(user),
            &vec! [AuthError::TokenNotGenerated]
        ).await);
    }

    let session_token = session_token.unwrap();

    match service_data.session.insert("token", session_token.key.clone().unwrap()) {
        Ok(_) => Ok(HttpResponse::Found()
            .insert_header((header::LOCATION, "/"))
            .finish()),
        Err(_) => Ok(generate_login_page(&service_data,
            Option::from(user),
            &vec! [AuthError::CookieNotWrote]
        ).await)
    }
}

#[get("/logout")]
async fn get_logout(req: HttpRequest,
                   context: web::Data<Context>,
                   session: Session
) -> actix_web::Result<HttpResponse> {
    let service_data = ServiceData::new(req, context, session).await;
    if(!services::users::is_authored(&service_data).await) {
        return utils::errors::page_403(&service_data).await;
    }

    let token = match services::users::get_current_token(&service_data).await {
        Ok(t) => t,
        Err(e) => return utils::errors::page_500(&service_data).await
    };

    service_data.session.clear();

    match models::session::Session::remove_by_token(&service_data.context.db, token.as_str()).await {
        Ok(_) => Ok(HttpResponse::Found()
            .insert_header((header::LOCATION, "/"))
            .finish()),
        Err(_) => utils::errors::page_500(&service_data).await
    }


}