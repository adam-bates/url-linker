use rocket::{http::Status, response, response::Responder, Request};

pub enum UrlError {
    KeyAlreadyExists,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for UrlError {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'o> {
        return match self {
            Self::KeyAlreadyExists => response::Result::Err(Status::BadRequest),
            _ => response::Result::Err(Status::InternalServerError),
        };
    }
}
