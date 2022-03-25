#[derive(Debug)]
pub struct CreateUserRequest {
    pub client_id: String,
    pub client_secret: String,
    pub is_admin: Option<bool>,
}

#[derive(Debug)]
pub struct UpdateUserRequest {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub is_admin: Option<bool>,
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub client_id: String,
    pub is_admin: bool,
}
