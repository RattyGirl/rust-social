use rust_social::{Request, User};

pub fn login_post(request: &Request) -> (String, String) {
    let parsed = json::parse(request.body.as_str());
    match parsed {
        Ok(v) => {
            if v["username"].is_null() || v["password"].is_null() {
                (
                    "HTTP/1.1 400 BAD REQUEST".to_string(),
                    "Invalid JSON".to_string(),
                )
            } else {
                match User::login(v["username"].as_str().unwrap(), v["password"].as_str().unwrap()) {
                    Some(u) => {
                        let token = u.generate_token().unwrap_or("".to_string());
                        (
                            "HTTP/1.1 200 OK\nSet-Cookie: token=".to_string()
                                + token.as_str(),
                            "Hello ".to_string() + u.username.as_str(),
                        )
                    }
                    None => {
                        ("HTTP/1.1 200 OK".to_string(), "Unable to login user".to_string())
                    }
                }
            }
        }
        Err(_) => (
            "HTTP/1.1 400 BAD REQUEST".to_string(),
            "Invalid JSON".to_string(),
        ),
    }

}

pub fn register_post(request: &Request) -> (String, String) {
    let parsed = json::parse(request.body.as_str());
    match parsed {
        Ok(v) => {
            if v["username"].is_null() || v["password"].is_null() {
                (
                    "HTTP/1.1 400 BAD REQUEST".to_string(),
                    "Invalid JSON".to_string(),
                )
            } else {
                match User::new(v["username"].as_str().unwrap(), v["password"].as_str().unwrap()) {
                    Some(u) => {
                        let token = u.generate_token().unwrap_or("".to_string());

                        (
                            "HTTP/1.1 200 OK\nSet-Cookie: token=".to_string()
                                + token.as_str(),
                            "Hello ".to_string() + u.username.as_str(),
                        )
                    }
                    None => {
                        ("HTTP/1.1 200 OK".to_string(), "Unable to register user".to_string())
                    }
                }
            }
        }
        Err(_) => (
            "HTTP/1.1 400 BAD REQUEST".to_string(),
            "Invalid JSON".to_string(),
        ),
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