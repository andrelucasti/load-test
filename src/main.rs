mod service_controller;
mod service;
extern crate rocket;
use rocket::{routes};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![service_controller::run_test, service_controller::hello])
}