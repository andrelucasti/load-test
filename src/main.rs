mod service_controller;
mod service_request;
mod service;

extern crate rocket;
use rocket::{routes};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![service_controller::run_test])
}