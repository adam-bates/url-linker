use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use crate::services::{types::user::User, user::UserService};
use crate::utils;

lazy_static! {
    static ref HEADER_CLIENT_ID: String = utils::required_env_var("HEADER_CLIENT_ID");
    static ref HEADER_CLIENT_SECRET: String = utils::required_env_var("HEADER_CLIENT_SECRET");
}

#[derive(Debug)]
pub enum UserCredentialsError {
    Missing,
    Invalid,
    NoUserService,
    Unknown,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserCredentialsError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = req.headers();

        // Get client credentials from request headers
        let (client_id, client_secret) = match (
            headers.get_one(HEADER_CLIENT_ID.as_str()),
            headers.get_one(HEADER_CLIENT_SECRET.as_str()),
        ) {
            (Some(client_id), Some(client_secret)) => {
                (String::from(client_id), String::from(client_secret))
            }
            _ => {
                return Outcome::Failure((Status::Unauthorized, UserCredentialsError::Missing));
            }
        };

        // Get UserService from request guards
        let user_service = match get_user_service(req).await {
            Outcome::Success(user_service) => user_service,
            Outcome::Failure(e) => return Outcome::Failure(e),
            _ => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    UserCredentialsError::Unknown,
                ))
            }
        };

        // Get verified user
        return match user_service.verify_and_get(client_id, client_secret).await {
            Ok(user) => Outcome::Success(user),
            _ => Outcome::Failure((Status::Unauthorized, UserCredentialsError::Invalid)),
        };
    }
}

async fn get_user_service(
    req: &Request<'_>,
) -> Outcome<Box<dyn UserService>, UserCredentialsError> {
    let user_service_outcome = req.guard::<Box<dyn UserService>>().await;

    if let Outcome::Success(user_service) = user_service_outcome {
        return Outcome::Success(user_service);
    } else {
        return Outcome::Failure((
            Status::InternalServerError,
            UserCredentialsError::NoUserService,
        ));
    }
}
