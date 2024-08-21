use actix_session::Session;
use actix_web::{HttpRequest, web};
use crate::Context;
use crate::core::models::DbError;

pub struct ServiceData<'a> {
    pub req: HttpRequest,
    pub context: web::Data<Context<'a>>,
    pub session: Session
}

impl ServiceData<'_> {
    pub(crate) async fn new(req: HttpRequest, context: web::Data<Context<'_>>, session: Session) -> Result<ServiceData, DbError> {
        let data = ServiceData {
            req,
            context,
            session
        };
        Ok(data)
    }
}