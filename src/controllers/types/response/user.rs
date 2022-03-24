use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: String,
    pub client_id: String,
}

#[derive(Debug, Serialize)]
pub struct Users {
    pub values: Vec<User>,
}

impl From<Vec<User>> for Users {
    fn from(values: Vec<User>) -> Self {
        return Self { values };
    }
}
