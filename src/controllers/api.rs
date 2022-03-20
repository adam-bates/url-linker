use std::path::PathBuf;

use rocket::{
    routes,
    serde::{json::Json, Deserialize},
    Build, Rocket,
};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount(
        "/api",
        routes![
            health,
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

#[derive(Deserialize)]
struct Url {
    bucket: String,
    key: String,
    destination: String,
}

#[post("/urls", data = "<url>")]
fn create(url: Json<Url>) -> String {
    return format!("create");
}

#[get("/urls")]
fn get_all() -> String {
    return format!("get_all");
}

#[get("/urls/<input..>")]
fn get_by_key(input: PathBuf) -> String {
    return format!("get_by_key: {:?}", input);
}

#[put("/urls/<input..>", data = "<url>")]
fn update_by_key(input: PathBuf, url: Json<Url>) -> String {
    return format!("update_by_key: {:?}", input);
}

#[delete("/urls/<input..>", data = "<url>")]
fn delete_by_key(input: PathBuf, url: Json<Url>) -> String {
    return format!("delete_by_key: {:?}", input);
}
