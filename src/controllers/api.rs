use std::path::PathBuf;

use rocket::{http::Status, routes, serde::json::Json, Build, Rocket};

use crate::config::database::DbConnection;
use crate::errors::UrlError;
use crate::services::UrlService;
use crate::types::Url;

use super::guards::ApiUser;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount(
        "/api",
        routes![
            health,
            index,
            create,
            get_all,
            get_by_key,
            update_by_key,
            delete_by_key
        ],
    );
}

#[get("/health")]
async fn health() -> &'static str {
    return "Service is healthy.";
}

#[get("/")]
async fn index() -> Status {
    return Status::NotFound;
}

#[post("/urls", data = "<url>")]
async fn create(service: UrlService, url: Json<Url>, _api_user: ApiUser) -> Result<(), UrlError> {
    // TODO: Validate

    service.create(url.into()).await?;

    return Ok(());
}

#[get("/urls")]
async fn get_all(_api_user: ApiUser) -> String {
    return format!("get_all");
}

#[get("/urls/<key..>")]
async fn get_by_key(db: DbConnection, key: PathBuf, _api_user: ApiUser) -> String {
    let key = key.display().to_string();

    return db
        .run(move |connection| {
            for row in connection
                .query("SELECT key, url FROM key_urls WHERE key = $1;", &[&key])
                .unwrap()
            {
                let url: &str = row.get("url");

                return String::from(url);
            }

            panic!("Not found!");
        })
        .await;
}

#[put("/urls/<key..>", data = "<url>")]
async fn update_by_key(key: PathBuf, url: Json<Url>, _api_user: ApiUser) -> String {
    let key = key.display().to_string();
    let url = url.0;

    return format!("update_by_key: {key}\nBody: {url:?}");
}

#[delete("/urls/<key..>")]
async fn delete_by_key(key: PathBuf, _api_user: ApiUser) -> String {
    let key = key.display().to_string();

    return format!("delete_by_key: {key}");
}
