use rocket::post;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::service::Service;
use crate::service_request::ServiceRequest;

#[post("/service", format = "application/json",  data = "<request>")]
pub fn run_test(request: Json<ServiceRequest>) -> status::Accepted<String> {
    println!("request: {:?}", request);

    let service = Service::new(
        request.get_url(),
        request.get_payload(),
        request.get_number_of_requests()
    );

    println!("service: {:?}", service);

    status::Accepted::<String>(Some(String::from("ok")))
}