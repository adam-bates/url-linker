use std::path::PathBuf;

use rocket::{http::uri::Reference, response::Redirect, routes, Build, Rocket};

use crate::errors::url::UrlError;
use crate::services::url::UrlService;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount("/", routes![query]);
}

#[get("/<key..>", rank = 2)]
async fn query(url_service: Box<dyn UrlService>, key: PathBuf) -> Result<Redirect, UrlError> {
    let key = key.display().to_string();

    let url = url_service.get_by_key(key).await?;

    let reference = Reference::try_from(url.url).map_err(|_| UrlError::UnexpectedUrlParseError)?;

    return Ok(Redirect::to(reference));
}
