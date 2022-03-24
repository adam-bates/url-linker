use rocket::serde::{json::Json, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Url {
    pub key: String,
    pub url: String,
}

impl From<Json<Url>> for Url {
    fn from(json: Json<Url>) -> Self {
        return json.0;
    }
}
