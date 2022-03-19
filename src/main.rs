extern crate dotenv;

use dotenv::dotenv;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn init_env() {
    match dotenv() {
        Ok(_) => {}
        Err(dotenv::Error::Io(io_err)) if io_err.kind() == std::io::ErrorKind::NotFound => {}
        err => panic!("Error! {:?}", err),
    };
}

#[launch]
fn rocket() -> _ {
    init_env();

    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }

    rocket::build().mount("/", routes![index])
}
