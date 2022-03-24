use rocket::{http::Status, routes, Build, Rocket};

mod url_crud;
// mod user_management;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/api", routes![health, index]);

    let rocket = url_crud::mount(rocket);
    // let rocket = user_management::mount(rocket);

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
