use rust_social::Request;
use std::fs;
use crate::user;

pub fn admin_get(request: &Request) -> (String, String) {
    let username: String;

    match user::verify_user(&request) {
        Some(u) => {
            username = u;
        }
        _ => {
            return (
                "HTTP/1.1 400 BAD REQUEST".to_string(),
                "No authentication".to_string(),
            )
        }
    }

    if user::does_user_have_role(username, "admin".to_string()) {
        (
            "HTTP/1.1 200 OK".to_string(),
            fs::read_to_string("www/admin.html").unwrap_or("admin".to_string()),
        )
    } else {
        (
            "HTTP/1.1 400 BAD REQUEST".to_string(),
            "No authentication".to_string(),
        )
    }
}
