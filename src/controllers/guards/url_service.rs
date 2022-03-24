use rocket::request::{FromRequest, Outcome, Request};

use crate::config::database::DbConnection;
use crate::services::url::{DbUrlService, UrlService};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Box<dyn UrlService> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        return match req.guard::<DbConnection>().await {
            Outcome::Success(db) => Outcome::Success(DbUrlService::new(db)),
            Outcome::Failure(e) => Outcome::Failure(e),
            Outcome::Forward(e) => Outcome::Forward(e),
        };
    }
}
