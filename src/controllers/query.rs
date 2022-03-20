use std::path::PathBuf;

use rocket::{routes, Build, Rocket};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount("/", routes![query]);
}

#[get("/<query..>")]
fn query(query: PathBuf) -> String {
    return format!("query: {:?}", query);
}
