use std::collections::HashMap;
use rust_social::{make_view, User};
use crate::{Request, Response};

pub fn login_post(request: &Request) -> Response {

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
                        Response::new().with_code(200)
                            .with_header("Set-Cookie".to_string(), format!("token={}", token.as_str()))
                            .with_body(make_view!("homeredirect.html").to_string()).clone()
                    }
                    None => {
                        Response::new().with_code(200).with_body("Unable to login user".to_string()).clone()
                    }
                }
            }
            (_, _) => {
                Response::new().with_code(400).with_body("Invalid Parameters".to_string()).clone()
            }
        }
    } else {
        Response::new().with_code(400).with_body("Invalid Parameters".to_string()).clone()
    }

}

pub fn register_get(_request: &Request) -> Response {
    Response::new().with_code(200).with_body(make_view!("user/register.html").to_string()).clone()
}

pub fn register_post(request: &Request) -> Response {
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
                        Response::new().with_code(200)
                            .with_header("Set-Cookie".to_string(), format!("token={}", token.as_str()))
                            .with_body(make_view!("homeredirect.html").to_string()).clone()
                    }
                    None => {
                        Response::new().with_code(200).with_body("Unable to register user".to_string()).clone()
                    }
                }
            }
            (_, _) => {
                Response::new().with_code(400).with_body("Invalid Parameters".to_string()).clone()
            }
        }
    } else {
        Response::new().with_code(400).with_body("Invalid Parameters".to_string()).clone()
    }
}

#[cfg(test)]
mod tests {
    use rust_social::DB_LOCATION;
    use rusqlite::Connection;

    fn setup() {
        let conn = Connection::open(DB_LOCATION).unwrap();
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
