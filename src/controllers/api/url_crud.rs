use std::path::PathBuf;

use rocket::{routes, serde::json::Json, Build, Rocket};

use crate::errors::url::UrlError;
use crate::services::{types::user::User, url::UrlService};

use super::super::types::{
    request::url::Url as UrlRequest,
    response::url::{Url, Urls},
};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount(
        "/api/urls",
        routes![create, get_all, get_by_key, update_by_key, delete_by_key],
    );
}

#[post("/", data = "<url>")]
async fn create(
    url_service: Box<dyn UrlService>,
    url: Json<UrlRequest>,
    user: User,
) -> Result<Url, UrlError> {
    let url: UrlRequest = url.0;

    url_service.create(user, url.into()).await?;

    todo!();
}

#[get("/")]
async fn get_all(url_service: Box<dyn UrlService>, user: User) -> Result<Urls, UrlError> {
    url_service.get_all_by_user(user).await?;

    todo!();
}

#[get("/<key..>")]
async fn get_by_key(
    url_service: Box<dyn UrlService>,
    key: PathBuf,
    user: User,
) -> Result<Url, UrlError> {
    let key = key.display().to_string();

    url_service.get_by_key_for_user(user, key).await?;

    todo!();
}

#[put("/<key..>", data = "<url>")]
async fn update_by_key(
    url_service: Box<dyn UrlService>,
    key: PathBuf,
    url: Json<UrlRequest>,
    user: User,
) -> Result<Url, UrlError> {
    let key = key.display().to_string();
    let url: UrlRequest = url.0;

    url_service
        .update_by_key_for_user(user, key, url.into())
        .await?;

    todo!();
}

#[delete("/<key..>")]
async fn delete_by_key(
    url_service: Box<dyn UrlService>,
    key: PathBuf,
    user: User,
) -> Result<(), UrlError> {
    let key = key.display().to_string();

    url_service.delete_by_key_for_user(user, key).await?;

    todo!();
}
