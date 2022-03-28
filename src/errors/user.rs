use rocket::{
    http::Status,
    response::{Responder, Result},
    serde::json::Json,
    Request,
};

use serde::Serialize;

use super::url::UrlError;

#[derive(Debug, Serialize)]
pub enum UserError {
    ClientIdAlreadyExists,
    ClientIdTooShort { min: usize },
    ClientIdTooLong { max: usize },
    ClientSecretTooShort { min: usize },
    ClientSecretTooLong { max: usize },
    UrlDeletionError(UrlError),
    Invalid,
    NotFound,
    HashError(String),
    Unknown,
}

impl UserError {
    fn bad_request<'r, 'o>(self, request: &'r Request<'_>) -> Result<'o> {
        return Responder::respond_to(Json(self), request).map(|mut res| {
            res.set_status(Status::BadRequest);
            return res;
        });
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for UserError {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        return match self {
            Self::ClientIdAlreadyExists
            | Self::ClientIdTooShort { .. }
            | Self::ClientIdTooLong { .. }
            | Self::ClientSecretTooShort { .. }
            | Self::ClientSecretTooLong { .. } => self.bad_request(request),

            Self::Invalid => Err(Status::Unauthorized),

            Self::NotFound => Err(Status::NotFound),

            _ => Err(Status::InternalServerError),
        };
    }
}
