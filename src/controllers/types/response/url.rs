use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Url {
    pub key: String,
    pub url: String,
    pub user_id: isize,
}

#[derive(Debug, Serialize)]
pub struct Urls {
    pub key: String,
    pub url: String,
}
