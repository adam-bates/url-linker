use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use crate::services::types::{admin::Admin, user::User};

use super::user::UserCredentialsError;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = UserCredentialsError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        return match req.guard::<User>().await {
            Outcome::Success(user) if user.is_admin => Outcome::Success(Admin(user)),
            Outcome::Success(_) => {
                Outcome::Failure((Status::Forbidden, UserCredentialsError::Invalid))
            }
            Outcome::Failure(e) => Outcome::Failure(e),
            Outcome::Forward(e) => Outcome::Forward(e),
        };
    }
}
