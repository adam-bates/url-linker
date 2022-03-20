use std::path::PathBuf;

use rocket::{
    http::Status,
    routes,
    serde::{json::Json, Deserialize},
    Build, Rocket,
};

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
fn health() -> &'static str {
    return "Service is healthy.";
}

#[get("/")]
fn index() -> Status {
    return Status::NotFound;
}

#[derive(Debug, Deserialize)]
struct Url {
    key: String,
    url: String,
}

#[post("/urls", data = "<url>")]
fn create(url: Json<Url>) -> String {
    let url = url.0;

    return format!("create\nBody: {url:?}");
}

#[get("/urls")]
fn get_all() -> String {
    return format!("get_all");
}

#[get("/urls/<key..>")]
fn get_by_key(key: PathBuf) -> String {
    let key = key.display().to_string();

    return format!("get_by_key: {key}");
}

#[put("/urls/<key..>", data = "<url>")]
fn update_by_key(key: PathBuf, url: Json<Url>) -> String {
    let key = key.display().to_string();
    let url = url.0;

    return format!("update_by_key: {key}\nBody: {url:?}");
}

#[delete("/urls/<key..>")]
fn delete_by_key(key: PathBuf) -> String {
    let key = key.display().to_string();

    return format!("delete_by_key: {key}");
}
