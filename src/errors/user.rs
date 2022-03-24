use rocket::{http::Status, response, response::Responder, Request};

pub enum UserError {
    ClientIdAlreadyExists,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for UserError {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'o> {
        return match self {
            Self::ClientIdAlreadyExists => response::Result::Err(Status::BadRequest),
            _ => response::Result::Err(Status::InternalServerError),
        };
    }
}
