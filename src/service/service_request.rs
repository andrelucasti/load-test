use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceRequest {
    url: String,
    payload: String,
    number_of_requests: u32, // maybe number of request by second
}

impl ServiceRequest {
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_payload(&self) -> String {
        self.payload.clone()
    }

    pub fn get_number_of_requests(&self) -> u32 {
        self.number_of_requests.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_new_service() {
        let service = ServiceRequest {
            url: String::from("https:service-order.com/api/v1/orders"),
            payload: String::from("{\"name\": \"order 1\",
                \"description\": \"order 1 description\",
                \"price\": 100\
            }"),
            number_of_requests: 1,
        };
        assert_eq!(service.url, "https:service-order.com/api/v1/orders");
        assert_eq!(service.payload, "{\"name\": \"order 1\",
                \"description\": \"order 1 description\",
                \"price\": 100\
            }");

        assert_eq!(service.number_of_requests, 1)
    }
}

