use rocket::request::{FromRequest, Outcome, Request};

use crate::config::database::DbConnection;
use crate::services::{
    password::{Argon2Config, Argon2ConfigRef, Argon2PasswordService},
    url::UrlService,
    user::{DbUserService, UserService},
};
use crate::utils;

lazy_static! {
    static ref HASHER_SECRET: String = utils::required_env_var("HASHER_SECRET");
    static ref ARGON2_CONFIG: Argon2ConfigRef = build_argon2_config_ref();
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Box<dyn UserService> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        return match (
            req.guard::<DbConnection>().await,
            req.guard::<Box<dyn UrlService>>().await,
        ) {
            (Outcome::Success(db), Outcome::Success(url_service)) => {
                let argon2_config = Argon2ConfigRef::clone(&ARGON2_CONFIG);

                Outcome::Success(DbUserService::new(
                    db,
                    url_service,
                    Argon2PasswordService::new(argon2_config),
                ))
            }
            (Outcome::Failure(e), _) | (_, Outcome::Failure(e)) => Outcome::Failure(e),
            (Outcome::Forward(e), _) | (_, Outcome::Forward(e)) => Outcome::Forward(e),
        };
    }
}

fn build_argon2_config_ref() -> Argon2ConfigRef {
    return Argon2ConfigRef::new(Argon2Config {
        secret: HASHER_SECRET.as_bytes(),
        variant: argon2::Variant::Argon2id,
        ..Argon2Config::default()
    });
}
