use rocket::serde::Serialize;

use crate::services::types::url::Url as ServiceUrl;

#[derive(Debug, Serialize)]
pub struct Url {
    pub key: String,
    pub url: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct Urls {
    pub values: Vec<Url>,
}

impl From<Vec<Url>> for Urls {
    fn from(values: Vec<Url>) -> Self {
        return Self { values };
    }
}

impl From<ServiceUrl> for Url {
    fn from(url: ServiceUrl) -> Self {
        return Self {
            key: url.key,
            url: url.url,
            user_id: url.user_id,
        };
    }
}
