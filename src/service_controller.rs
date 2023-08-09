use rocket::post;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::service::Service;

#[post("/service", format = "application/json",  data = "<new_service>")]
pub fn run_test(new_service: Json<Service>) -> status::Accepted<String> {
    println!("New service: {:?}", new_service);
    status::Accepted(Some(String::from("Accepted")))

}