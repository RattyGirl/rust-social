use std::collections::HashMap;
use rust_social::{make_view, Request, User};

pub fn login_post(request: &Request) -> (String, String) {

    let mut parameters_hashmap: HashMap<String, String> = HashMap::new();

    if request.headers.get("Content-Type").unwrap_or(&String::new()).eq("application/x-www-form-urlencoded") {
        let parameters: Vec<&str> = request.body.split('&').collect();
        for parameter in parameters {
            if !parameter.is_empty() {
                let vec: Vec<&str> = parameter.split('=').collect();
                if vec.len() == 2 {
                    parameters_hashmap.insert(vec[0].to_string(), vec[1].to_string());
                } else {
                    parameters_hashmap.insert(vec[0].to_string(), String::new());
                }
            }
        }

        match (parameters_hashmap.get("username"), parameters_hashmap.get("password")) {
            (Some(username), Some(password)) => {
                match User::login(username, password) {
                    Some(u) => {
                        let token = u.generate_token().unwrap_or_default();
                        (
                            "HTTP/1.1 200 OK\nSet-Cookie: token=".to_string()
                                + token.as_str(),
                            make_view!("homeredirect.html").to_string(),
                        )
                    }
                    None => {
                        ("HTTP/1.1 200 OK".to_string(), "Unable to login user".to_string())
                    }
                }
            }
            (_, _) => {
                (
                    "HTTP/1.1 400 BAD REQUEST".to_string(),
                    "Invalid Parameters".to_string(),
                )
            }
        }
    } else {
        (
            "HTTP/1.1 400 BAD REQUEST".to_string(),
            "Invalid Parameters".to_string(),
        )
    }

}

pub fn register_get(_request: &Request) -> (String, String) {
    (
        "HTTP/1.1 200 OK".to_string(),
        make_view!("user/register.html").to_string(),
    )
}

pub fn register_post(request: &Request) -> (String, String) {
    let mut parameters_hashmap: HashMap<String, String> = HashMap::new();

    if request.headers.get("Content-Type").unwrap_or(&String::new()).eq("application/x-www-form-urlencoded") {
        let parameters: Vec<&str> = request.body.split('&').collect();
        for parameter in parameters {
            if !parameter.is_empty() {
                let vec: Vec<&str> = parameter.split('=').collect();
                if vec.len() == 2 {
                    parameters_hashmap.insert(vec[0].to_string(), vec[1].to_string());
                } else {
                    parameters_hashmap.insert(vec[0].to_string(), String::new());
                }
            }
        }

        match (parameters_hashmap.get("username"), parameters_hashmap.get("password")) {
            (Some(username), Some(password)) => {
                match User::new(username, password) {
                    Some(u) => {
                        let token = u.generate_token().unwrap_or_default();

                        (
                            "HTTP/1.1 200 OK\nSet-Cookie: token=".to_string()
                                + token.as_str(),
                            make_view!("homeredirect.html").to_string(),
                        )
                    }
                    None => {
                        ("HTTP/1.1 200 OK".to_string(), "Unable to register user".to_string())
                    }
                }
            }
            (_, _) => {
                (
                    "HTTP/1.1 400 BAD REQUEST".to_string(),
                    "Invalid Parameters".to_string(),
                )
            }
        }
    } else {
        (
            "HTTP/1.1 400 BAD REQUEST".to_string(),
            "Invalid Parameters".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use rust_social::{Request, TYPE};
    use crate::user::{register_post};
    use rusqlite::Connection;

    fn setup() {
        let conn = Connection::open("rust-social.db").unwrap();
        crate::generate_tables(&conn);
        conn.close().unwrap();
    }

    #[test]
    fn register_user_exists() {
        setup();

    }

    #[test]
    fn register_successful() {
        setup();

    }
}
