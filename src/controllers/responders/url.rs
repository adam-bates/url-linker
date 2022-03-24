use rocket::{
    response::{Responder, Result as RocketResult},
    serde::json::Json,
    Request,
};

use super::super::types::response::url::{Url, Urls};

impl<'r, 'o: 'r> Responder<'r, 'o> for Url {
    fn respond_to(self, request: &'r Request<'_>) -> RocketResult<'o> {
        return Json::from(self).respond_to(request);
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Urls {
    fn respond_to(self, request: &'r Request<'_>) -> RocketResult<'o> {
        return Json::from(self).respond_to(request);
    }
}
