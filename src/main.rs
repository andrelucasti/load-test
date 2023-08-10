extern crate rocket;
use rocket::{routes};
mod service;
use service::service_controller;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![service_controller::run_test])
}