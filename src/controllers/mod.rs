use rocket::{Build, Rocket};

mod api;
mod query;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = api::mount(rocket);
    let rocket = query::mount(rocket);

    return rocket;
}
