use rust_social::{User};
use crate::{make_view, Request, Response};

pub fn admin_get(request: &Request) -> Response {
    match User::find_user(request.cookies.get("token").unwrap_or(&String::new())) {
        Some(u) => {
            if u.does_user_have_role("admin".to_string()) {
                Response::default().with_code(200).with_body(make_view!("admin.social").to_string()).clone()
            } else {
                Response::default().with_code(200).with_body("You are not able to access".to_string()).clone()
            }
        }
        None => {
            Response::default().with_code(200).with_body(make_view!("homeredirect.social").to_string()).clone()
        }
    }
}
