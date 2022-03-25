use rocket::{
    response::{Responder, Result as RocketResult},
    serde::json::Json,
    Request,
};

use super::super::types::response::user::{User, Users};

impl<'r, 'o: 'r> Responder<'r, 'o> for User {
    fn respond_to(self, request: &'r Request<'_>) -> RocketResult<'o> {
        return Json::from(self).respond_to(request);
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Users {
    fn respond_to(self, request: &'r Request<'_>) -> RocketResult<'o> {
        return Json::from(self).respond_to(request);
    }
}
