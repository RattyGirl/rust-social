use super::httprequest;
use rusqlite::{Connection, params, Error};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub hashedpw: String
}

pub fn login(request: httprequest::Request, buffer: [u8; 1024]) -> (String, String) {

    if request.req_type == httprequest::TYPE::POST {
        let parsed = json::parse(request.body.as_str());
        match parsed {
            Ok(v) => {
                if v["username"].is_null() || v["password"].is_null() {
                    return ("HTTP/1.1 400 BAD REQUEST\r\n".to_string(), "Invalid JSON".to_string());
                } else {
                    let conn = Connection::open("rust-social.db").unwrap();
                    // TODO actually properly do authentication stuff

                    let row: Result<User, Error> = conn.query_row("SELECT * FROM users WHERE username= ?1",
                              params![v["username"].as_str()], |row| {
                        Ok(User {
                            username: row.get(0)?,
                            hashedpw: row.get(1)?
                        })
                    });
                    // todo check for auth
                    match row {
                        Ok(r) => {
                            let hashedpw = to_hash(v["username"].as_str().unwrap(), v["password"].as_str().unwrap());
                            return if check_user_password(v["username"].as_str().unwrap(), hashedpw.as_str(), &r) {
                                (
                                    "HTTP/1.1 200 OK\r\nSet-Cookie: token=".to_string() +
                                        hashedpw.as_str(),
                                    "Hello ".to_string() + v["username"].as_str().unwrap()
                                )
                            } else {
                                ("HTTP/1.1 400 BAD REQUEST\r\n".to_string(), "Invalid username or password".to_string())
                            }
                        }
                        Err(e) => {
                            return ("HTTP/1.1 400 BAD REQUEST\r\n".to_string(), e.to_string());
                        }
                    }
                }
            },
            Err(e) =>
                return ("HTTP/1.1 400 BAD REQUEST\r\n".to_string(), "Invalid JSON".to_string())
        }
    } else {
        return ("HTTP/1.1 404 NOT FOUND\r\n".to_string(), "pretend there a register page here".to_string());
    }
}


pub fn register(request: httprequest::Request, buffer: [u8; 1024]) -> (String, String) {
    if request.req_type == httprequest::TYPE::POST {
        let parsed = json::parse(request.body.as_str());
        match parsed {
            Ok(v) => {
                if v["username"].is_null() || v["password"].is_null() {
                    return ("HTTP/1.1 400 BAD REQUEST\r\n".to_string(), "Invalid JSON".to_string());
                } else {
                    let conn = Connection::open("rust-social.db").unwrap();
                    // TODO actually properly do authentication stuff

                    let hashedpw = to_hash(v["username"].as_str().unwrap(), v["password"].as_str().unwrap());

                    match conn.execute(
                        "INSERT INTO users (username, hashedpw) VALUES (?1, ?2)",
                        params![v["username"].as_str(), hashedpw]
                    ) {
                        Ok(_) => {
                            return (
                                "HTTP/1.1 200 OK\r\nSet-Cookie: token=".to_string() +
                                    hashedpw.as_str(),
                                "Hello ".to_string() + v["username"].as_str().unwrap()
                            );
                        }
                        Err(e) => {
                            return ("HTTP/1.1 400 BAD REQUEST\r\n".to_string(), e.to_string());
                        }
                    }
                }
            },
            Err(e) =>
                return ("HTTP/1.1 400 BAD REQUEST\r\n".to_string(), "Invalid JSON".to_string())
        }
    } else {
        return ("HTTP/1.1 404 NOT FOUND\r\n".to_string(), "pretend there a register page here".to_string());
    }
}

fn check_user_password(username: &str, password: &str, user: &User) -> bool {
    if user.username.eq(username) {
    //     check password
        if user.hashedpw.eq(password) {
            return true;
        }
    }
    return false
}

fn to_hash(username: &str, password: &str) -> String {
    let mut hasher = Sha3::sha3_512();
    hasher.input_str(env!("secret"));
    hasher.input_str(username);
    hasher.input_str(password);

    hasher.result_str()
}