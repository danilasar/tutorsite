use actix_session::{Session, SessionGetError};
use serde_json::json;
use crate::core::service_data::ServiceData;
//use crate::models::user::get_user_by_token;

pub(crate) async fn wrap_page(service_data: &ServiceData,
                              content: &str,
                              title: Option<&str>)
                              -> String
{
    let requested_with = match service_data.req.headers().get("X-Requested-With") {
        Some(header_value) => { header_value.to_str().unwrap_or("") },
        None => { "" }
    };
    if requested_with == "XMLHttpRequest" {
        return content.to_string();
    }

    /*let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("wrap", include_str!("../views/wrap.hbs"))
        .unwrap();*/

    let mut data = json!({ "page": { "name": title.unwrap_or_default(), "content": content } });


    /*if let Ok(option) = service_data.session.get("token") {
        let option : Option<String> = option;
        if let Some(token) = option {
            if let Ok(user) = get_user_by_token(&service_data.context.db, token.as_str()).await {
                data["user"] = json!(user);
            }
        }
    };*/

    let wrap = service_data.context.handlebars.render("layout", &data).unwrap();

    return wrap;
}