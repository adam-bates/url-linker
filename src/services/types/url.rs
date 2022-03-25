#[derive(Debug)]
pub struct CreateUrlRequest {
    pub key: String,
    pub url: String,
}

#[derive(Debug)]
pub struct UpdateUrlRequest {
    pub key: Option<String>,
    pub url: Option<String>,
}

pub struct Url {
    pub key: String,
    pub url: String,
    pub user_id: i32,
}
