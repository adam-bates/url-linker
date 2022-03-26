use rocket::{
    fs::{relative, FileServer},
    http::Status,
    routes, Build, Rocket,
};

mod urls;
mod users;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/api/v1", routes![index, health]);

    let rocket = rocket.mount(
        "/api/v1/swagger",
        FileServer::from(relative!("resources/swagger")).rank(1),
    );

    let rocket = urls::mount(rocket);
    let rocket = users::mount(rocket);

    return rocket;
}

#[get("/")]
async fn index() -> Status {
    return Status::NotFound;
}

#[get("/health")]
async fn health() -> &'static str {
    return "Service is healthy.";
}
