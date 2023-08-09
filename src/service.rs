use rocket::log::private::debug;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    url: String,
    payload: String,
    number_of_requests: u32, // maybe number of request by second
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_new_service() {
        let service = Service {
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

