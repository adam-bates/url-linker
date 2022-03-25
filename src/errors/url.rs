use rocket::{
    http::Status,
    response::{Responder, Result},
    Request,
};

#[derive(Debug)]
pub enum UrlError {
    KeyAlreadyExists,
    NotFound,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for UrlError {
    fn respond_to(self, _request: &'r Request<'_>) -> Result<'o> {
        return match self {
            Self::KeyAlreadyExists => Err(Status::BadRequest),
            Self::NotFound => Err(Status::NotFound),
            // _ => Err(Status::InternalServerError),
        };
    }
}
