use actix_session::Session;
use actix_web::{HttpRequest, web};
use crate::Context;
use crate::core::models::DbError;

pub struct ServiceData {
    pub req: HttpRequest,
    pub context: web::Data<Context>,
    pub session: Session
}

impl ServiceData {
    pub(crate) async fn new(req: HttpRequest, context: web::Data<Context>, session: Session) -> ServiceData {
        ServiceData {
            req,
            context,
            session
        }
    }
}