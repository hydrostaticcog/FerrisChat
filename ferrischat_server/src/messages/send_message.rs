use rocket::http::Status;
use rocket::response::{content, status};

pub async fn send_message() -> status::Custom<&'static str> {
    status::Custom(Status::Created, "Send message test")
}
