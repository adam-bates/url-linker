use rocket::serde::{json::Json, Deserialize};

use crate::services::types::url::UrlRequest;

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

impl Into<UrlRequest> for Url {
    fn into(self) -> UrlRequest {
        return UrlRequest {
            key: self.key,
            url: self.url,
        };
    }
}
