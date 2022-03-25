use rocket::{
    http::Status,
    response::{Responder, Result},
    serde::json::Json,
    Request,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum UrlError {
    KeyAlreadyExists,
    KeyReserved { prefix: String },
    KeyTooShort { min: usize },
    KeyTooLong { max: usize },
    UrlParseError(String),
    UrlInvalid,
    NotFound,
    Unknown,
    UnexpectedUrlParseError,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for UrlError {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        return match self {
            Self::KeyAlreadyExists
            | Self::KeyTooShort { .. }
            | Self::KeyTooLong { .. }
            | Self::UrlParseError(_)
            | Self::UrlInvalid => Responder::respond_to(Json(self), request),
            Self::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        };
    }
}
