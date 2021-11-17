use rust_social::{Request, User};
use crate::{user, make_view};

pub fn admin_get(request: &Request) -> (String, String) {
    match User::get_if_valid(request.cookies.get("token").unwrap_or(&"".to_string())) {
        Some(u) => {
            if u.does_user_have_role("admin".to_string()) {
                (
                    "HTTP/1.1 200 OK".to_string(),
                    make_view!("admin.html").to_string(),
                )
            } else {
                (
                    "HTTP/1.1 200 OK".to_string(),
                    "You are not able to access".to_string()
                )
            }
        }
        None => {
            (
                "HTTP/1.1 200 OK".to_string(),
                make_view!("loginredirect.html").to_string()
            )
        }
    }
}
