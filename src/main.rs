#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

extern crate argon2;

mod config;
mod controllers;
mod errors;
mod services;
mod utils;

use config::{database::DbConnection, environment};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    environment::init_env();

    let rocket = rocket::build();
    let rocket = rocket.attach(DbConnection::fairing());
    let rocket = controllers::mount(rocket);

    return rocket.launch().await;
}
