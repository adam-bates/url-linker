extern crate dotenv;

use dotenv::dotenv;

pub fn init_env() {
    match dotenv() {
        Ok(_) => {}
        Err(dotenv::Error::Io(io_err)) if io_err.kind() == std::io::ErrorKind::NotFound => {}
        err => panic!("Error! {:?}", err),
    };

    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}
