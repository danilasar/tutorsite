pub enum DbError {
    NotFound,
    InvalidData
}

trait GetAll {
    async fn get_all() -> Result<Vec<Self>, DbError>;
}