use std::path::PathBuf;

use rocket::{routes, Build, Rocket};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket.mount("/", routes![query]);
}

#[get("/<key..>")]
fn query(key: PathBuf) -> String {
    let key = key.display().to_string();

    return format!("key: {key}");
}
