use rocket::request::{FromRequest, Outcome, Request};

use crate::config::database::DbConnection;
use crate::services::user::{DbUserService, UserService};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Box<dyn UserService> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        return match req.guard::<DbConnection>().await {
            Outcome::Success(db) => Outcome::Success(DbUserService::new(db)),
            Outcome::Failure(e) => Outcome::Failure(e),
            Outcome::Forward(e) => Outcome::Forward(e),
        };
    }
}
