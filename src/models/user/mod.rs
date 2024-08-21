
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub password_hash: Option<String>
}

impl User {

}