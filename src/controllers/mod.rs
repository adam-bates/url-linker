use rocket::{
    fs::{relative, FileServer},
    Build, Rocket,
};

mod api;
mod cors;
mod guards;
mod query;
mod responders;
mod types;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = cors::attach(rocket);

    let rocket = rocket.mount(
        "/client",
        FileServer::from(relative!("resources/web")).rank(1),
    );

    let rocket = api::mount(rocket);
    let rocket = query::mount(rocket);

    return rocket;
}
