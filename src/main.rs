#[macro_use]
extern crate rocket;

mod config;
mod controllers;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    config::init_env();

    let rocket = rocket::build();
    let rocket = controllers::mount(rocket);

    return match rocket.launch().await {
        Err(err) => match err.kind() {
            rocket::error::ErrorKind::Collisions(collisions) => {
                for r in &collisions.routes {
                    println!("\nRoute1: {:?}\n\nRoute2: {:?}\n", r.0, r.1);
                }
                return Ok(());
            }
            _ => Err(err),
        },
        x => x,
    };
}
