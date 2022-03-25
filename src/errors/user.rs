use rocket::{
    http::Status,
    response::{Responder, Result},
    Request,
};

use argon2::Error as Argon2Error;

#[derive(Debug)]
pub enum UserError {
    ClientIdAlreadyExists,
    Invalid,
    NotFound,
    HashError(Argon2Error),
    Unknown,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for UserError {
    fn respond_to(self, _request: &'r Request<'_>) -> Result<'o> {
        return match self {
            Self::ClientIdAlreadyExists => Err(Status::BadRequest),
            Self::Invalid => Err(Status::Unauthorized),
            Self::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        };
    }
}
