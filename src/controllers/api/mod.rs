use rocket::{http::Status, routes, Build, Rocket};

mod urls;
mod users;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/api", routes![health, index]);

    let rocket = urls::mount(rocket);
    let rocket = users::mount(rocket);

    return rocket;
}

#[get("/health")]
async fn health() -> &'static str {
    return "Service is healthy.";
}

#[get("/")]
async fn index() -> Status {
    return Status::NotFound;
}
