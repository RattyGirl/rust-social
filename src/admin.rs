use rust_social::Request;
use crate::{user, make_view};

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
            make_view!("admin.html").to_string(),
        )
    } else {
        (
            "HTTP/1.1 400 BAD REQUEST".to_string(),
            "No authentication".to_string(),
        )
    }
}
