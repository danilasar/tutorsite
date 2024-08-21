pub enum DbError {
    NotFound,
    InvalidData,
    InternalError(sqlx::Error)
}

/*trait GetAll {
    async fn get_all() -> Result<Vec<Self>, DbError>;
}*/