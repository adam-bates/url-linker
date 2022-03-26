use std::path::PathBuf;

use rocket::{routes, serde::json::Json, Build, Rocket};

use crate::errors::url::UrlError;
use crate::services::types::admin::Admin;
use crate::services::{types::user::User, url::UrlService};

use super::super::types::{
    request::url::{CreateUrl, UpdateUrl},
    response::url::{Url, Urls},
};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount(
        "/api/v1/urls",
        routes![
            create,
            get_all_for_admin,
            get_all_by_user_id,
            get_by_key,
            update_by_key,
            delete_by_key
        ],
    );
}

#[post("/", data = "<url>")]
async fn create(
    user: User,
    url_service: Box<dyn UrlService>,
    url: Json<CreateUrl>,
) -> Result<Url, UrlError> {
    let url: CreateUrl = url.0;

    let url = url_service.create(user, url.into()).await?;

    return Ok(Url::from(url));
}

#[get("/?include_all", rank = 1)]
async fn get_all_for_admin(
    _admin: Admin,
    url_service: Box<dyn UrlService>,
) -> Result<Urls, UrlError> {
    let urls = url_service.get_all().await?;

    return Ok(Urls {
        values: urls.into_iter().map(|url| Url::from(url)).collect(),
    });
}

#[get("/?<user_id>", rank = 2)]
async fn get_all_by_user_id(
    user: User,
    url_service: Box<dyn UrlService>,
    user_id: Option<i32>,
) -> Result<Urls, UrlError> {
    let user_id = match user_id {
        Some(user_id) if user.is_admin => user_id,
        _ => user.id,
    };

    let urls = url_service.get_all_by_user_id(user_id).await?;

    return Ok(Urls {
        values: urls.into_iter().map(|url| Url::from(url)).collect(),
    });
}

#[get("/<key..>", rank = 3)]
async fn get_by_key(
    user: User,
    url_service: Box<dyn UrlService>,
    key: PathBuf,
) -> Result<Url, UrlError> {
    let key = key.display().to_string();

    let url = url_service.get_by_key_for_user(user, key).await?;

    return Ok(Url::from(url));
}

#[put("/<key..>", data = "<url>")]
async fn update_by_key(
    user: User,
    url_service: Box<dyn UrlService>,
    key: PathBuf,
    url: Json<UpdateUrl>,
) -> Result<Url, UrlError> {
    let key = key.display().to_string();
    let url: UpdateUrl = url.0;

    let url = url_service
        .update_by_key_for_user(user, key, url.into())
        .await?;

    return Ok(Url::from(url));
}

#[delete("/<key..>")]
async fn delete_by_key(
    user: User,
    url_service: Box<dyn UrlService>,
    key: PathBuf,
) -> Result<(), UrlError> {
    let key = key.display().to_string();

    url_service.delete_by_key_for_user(user, key).await?;

    return Ok(());
}
