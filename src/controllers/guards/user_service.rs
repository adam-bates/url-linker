use rocket::request::{FromRequest, Outcome, Request};

use crate::config::database::DbConnection;
use crate::services::password::{Argon2Config, Argon2ConfigRef, Argon2PasswordService};
use crate::services::user::{DbUserService, UserService};
use crate::utils;

lazy_static! {
    static ref HASHER_SECRET: String = utils::required_env_var("HASHER_SECRET");
    static ref ARGON2_CONFIG: Argon2ConfigRef = build_argon2_config_ref();
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Box<dyn UserService> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        return match req.guard::<DbConnection>().await {
            Outcome::Success(db) => {
                let argon2_config = Argon2ConfigRef::clone(&ARGON2_CONFIG);

                Outcome::Success(DbUserService::new(
                    db,
                    Argon2PasswordService::new(argon2_config),
                ))
            }
            Outcome::Failure(e) => Outcome::Failure(e),
            Outcome::Forward(e) => Outcome::Forward(e),
        };
    }
}

fn build_argon2_config_ref() -> Argon2ConfigRef {
    return Argon2ConfigRef::new(Argon2Config {
        secret: HASHER_SECRET.as_bytes(),
        ..Argon2Config::default()
    });
}
