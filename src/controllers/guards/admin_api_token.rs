use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use lazy_static;

use crate::utils;

use super::super::types::request::admin_api_token::AdminApiToken;

lazy_static! {
    static ref HEADER_ADMIN_API_TOKEN: String = utils::required_env_var("HEADER_ADMIN_API_TOKEN");
    static ref ADMIN_API_TOKEN: String = utils::required_env_var("ADMIN_API_TOKEN");
}

#[derive(Debug)]
pub enum AdminApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminApiToken {
    type Error = AdminApiTokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one(HEADER_ADMIN_API_TOKEN.as_str()) {
            Some(key) if is_valid(key) => Outcome::Success(AdminApiToken),
            Some(_) => Outcome::Failure((Status::Forbidden, AdminApiTokenError::Invalid)),
            None => Outcome::Failure((Status::Unauthorized, AdminApiTokenError::Missing)),
        }
    }
}

fn is_valid(key: &str) -> bool {
    return key == ADMIN_API_TOKEN.as_str();
}
