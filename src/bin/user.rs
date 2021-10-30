use super::httprequest;
use rusqlite::{Connection, params, Error};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

#[derive(Debug)]
pub struct User {
    pub username: String
}

pub fn does_user_have_role(username: String, role: String) -> bool {
    let conn = Connection::open("rust-social.db").unwrap();

    let result = conn.prepare("
            SELECT * FROM users_roles
            JOIN roles
            ON users_roles.role_id = roles.id
            WHERE username = ?1 AND name = ?2
        ").unwrap().exists(params![username, role]);

    match result {
        Ok(x) => x,
        Err(_) => false
    }


}

pub fn login(request: httprequest::Request, _buffer: [u8; 1024]) -> (String, String) {
    return if request.req_type == httprequest::TYPE::POST {
        let parsed = json::parse(request.body.as_str());
        match parsed {
            Ok(v) => {
                if v["username"].is_null() || v["password"].is_null() {
                    ("HTTP/1.1 400 BAD REQUEST".to_string(), "Invalid JSON".to_string())
                } else {
                    let hashedpw = to_hash(v["username"].as_str().unwrap(), v["password"].as_str().unwrap());

                    if check_user_token(v["username"].as_str().unwrap(), hashedpw.as_str()) {
                        (
                            "HTTP/1.1 200 OK\r\nSet-Cookie: token=".to_string() +
                                hashedpw.as_str() + "\nSet-Cookie: username=" + v["username"].as_str().unwrap(),
                            "Hello ".to_string() + v["username"].as_str().unwrap()
                        )
                    } else {
                        ("HTTP/1.1 400 BAD REQUEST".to_string(), "Invalid username or password".to_string())
                    }
                }
            },
            Err(_) =>
                ("HTTP/1.1 400 BAD REQUEST".to_string(), "Invalid JSON".to_string())
        }
    } else {
        ("HTTP/1.1 404 NOT FOUND".to_string(), "pretend there a register page here".to_string())
    }
}


pub fn register(request: httprequest::Request, _buffer: [u8; 1024]) -> (String, String) {
    return if request.req_type == httprequest::TYPE::POST {
        let parsed = json::parse(request.body.as_str());
        match parsed {
            Ok(v) => {
                if v["username"].is_null() || v["password"].is_null() {
                    ("HTTP/1.1 400 BAD REQUEST".to_string(), "Invalid JSON".to_string())
                } else {
                    let conn = Connection::open("rust-social.db").unwrap();
                    // TODO actually properly do authentication stuff



                    let hashedpw = to_hash(v["username"].as_str().unwrap(), v["password"].as_str().unwrap());

                    match conn.execute(
                        "INSERT INTO users (username, hashedpw) VALUES (?1, ?2)",
                        params![v["username"].as_str(), hashedpw]
                    ) {
                        Ok(_) => {
                            (
                                "HTTP/1.1 200 OK\r\nSet-Cookie: token=".to_string() +
                                    hashedpw.as_str() + "\nSet-Cookie: username=" + v["username"].as_str().unwrap(),
                                "Hello ".to_string() + v["username"].as_str().unwrap()
                            )
                        }
                        Err(e) => {
                            ("HTTP/1.1 400 BAD REQUEST".to_string(), e.to_string())
                        }
                    }
                }
            },
            Err(_) =>
                ("HTTP/1.1 400 BAD REQUEST".to_string(), "Invalid JSON".to_string())
        }
    } else {
        ("HTTP/1.1 404 NOT FOUND".to_string(), "pretend there a register page here".to_string())
    }
}

pub fn verify_user(request: &httprequest::Request) -> Option<String> {
    let username: String;
    let hashed: String;
    match (request.cookies.get("username"), request.cookies.get("token")) {
        (Some(u), Some(pw)) => {
            username = u.to_string();
            hashed = pw.to_string();
        }
        _ => {
            return None;
        }
    };

    return if check_user_token(username.as_str(), hashed.as_str()) {
        Some(username)
    } else {
        None
    }

}
pub fn check_user_token(username: &str, token: &str) -> bool {
    let conn = Connection::open("rust-social.db").unwrap();
    // TODO actually properly do authentication stuff maybe a per user salt

    let row: Result<(String, String), Error> = conn.query_row("SELECT * FROM users WHERE username= ?1",
                                                  params![username], |row| {
            return Ok(
                (
                    row.get(0)?,
                    row.get(1)?
                )
            );
        });

    match row {
        Ok(user) => {
            if user.0.eq(username) {
                if user.1.eq(token) {
                    return true;
                }
            }
        }
        Err(_) => {
            return false;
        }
    }
    return false;
}

fn to_hash(username: &str, password: &str) -> String {
    let mut rng = rand::thread_rng();

    let mut hasher = Sha3::sha3_512();
    hasher.input_str(env!("secret"));
    hasher.input_str(username);
    hasher.input_str(password);

    hasher.result_str()
}