use rocket::serde::{json::Json, Deserialize};

use crate::services::types::url::{CreateUrlRequest, UpdateUrlRequest};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUrl {
    pub key: String,
    pub url: String,
}

impl From<Json<CreateUrl>> for CreateUrl {
    fn from(json: Json<CreateUrl>) -> Self {
        return json.0;
    }
}

impl Into<CreateUrlRequest> for CreateUrl {
    fn into(self) -> CreateUrlRequest {
        return CreateUrlRequest {
            key: self.key,
            url: self.url,
        };
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUrl {
    pub key: Option<String>,
    pub url: Option<String>,
}

impl From<Json<UpdateUrl>> for UpdateUrl {
    fn from(json: Json<UpdateUrl>) -> Self {
        return json.0;
    }
}

impl Into<UpdateUrlRequest> for UpdateUrl {
    fn into(self) -> UpdateUrlRequest {
        return UpdateUrlRequest {
            key: self.key,
            url: self.url,
        };
    }
}
