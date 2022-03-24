use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use lazy_static;

use crate::config::database::DbConnection;
use crate::utils;

lazy_static! {
    static ref HEADER_CLIENT_ID: String = utils::requiredEnvVar("HEADER_CLIENT_ID");
    static ref HEADER_CLIENT_SECRET: String = utils::requiredEnvVar("HEADER_CLIENT_SECRET");
}

pub struct ApiUser {
    pub id: isize,
    pub client_id: String,
}

#[derive(Debug)]
pub enum ApiUserError {
    Missing,
    Invalid,
    NoDbConnection,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiUser {
    type Error = ApiUserError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = req.headers();

        match (
            headers.get_one(HEADER_CLIENT_ID.as_str()),
            headers.get_one(HEADER_CLIENT_SECRET.as_str()),
        ) {
            (Some(client_id), Some(client_secret)) => {
                let db_connection_outcome = req.guard::<DbConnection>().await;

                if let Outcome::Success(db_connection) = db_connection_outcome {
                    return match get_verified_user(db_connection, client_id, client_secret).await {
                        Some(user) => Outcome::Success(user),
                        _ => Outcome::Failure((Status::Forbidden, ApiUserError::Invalid)),
                    };
                } else {
                    return Outcome::Failure((
                        Status::InternalServerError,
                        ApiUserError::NoDbConnection,
                    ));
                }
            }
            _ => {
                return Outcome::Failure((Status::Unauthorized, ApiUserError::Missing));
            }
        }
    }
}

async fn get_verified_user(
    db_connection: DbConnection,
    client_id: &str,
    client_secret: &str,
) -> Option<ApiUser> {
    // TODO

    return None;
}
