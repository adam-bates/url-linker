#[macro_use]
extern crate rocket;

use rocket_sync_db_pools::{database, postgres};

mod config;
mod controllers;

#[database("url_linker")]
struct DbConnection(postgres::Client);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    config::init_env();

    let rocket = rocket::build();
    let rocket = rocket.attach(DbConnection::fairing());
    let rocket = controllers::mount(rocket);

    return rocket.launch().await;
}
