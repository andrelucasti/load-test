use uuid::Uuid;

#[derive(Debug)]
pub struct Service {
    id: Uuid,
    url: String,
    payload: String,
    number_of_requests: u32, // maybe number of request by second
}



impl Service {
    pub fn new(url: String, payload: String, number_of_requests: u32) -> Service {
        Service {
            id: Uuid::new_v4(),
            url,
            payload,
            number_of_requests,
        }
    }
}

