use rocket::serde::{json::Json, Deserialize};

#[derive(Debug, Deserialize)]
pub struct User {
    pub client_id: String,
    pub client_secret: String,
}

impl From<Json<User>> for User {
    fn from(json: Json<User>) -> Self {
        return json.0;
    }
}
