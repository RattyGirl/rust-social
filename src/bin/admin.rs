use super::httprequest;
use super::user;
use std::collections::HashMap;
use rusqlite::{Connection, params};
use rusqlite::ffi::Error;
use std::fs;

pub fn admin(request: httprequest::Request, _buffer: [u8; 1024]) -> (String, String) {
    let username: String;
    let hashed: String;
    match (request.cookies.get("username"), request.cookies.get("token")) {
        (Some(u), Some(pw)) => {
            username = u.to_string();
            hashed = pw.to_string();
        }
        _ => {
            return ("HTTP/1.1 400 BAD REQUEST".to_string(), "No authentication".to_string());
        }
    };

    if user::check_user_token(username.as_str(), hashed.as_str()) {
        // user and token match, time to check role
        {
            let conn = Connection::open("rust-social.db").unwrap();

            let mut result = conn.prepare("
                SELECT * FROM users_roles
                JOIN roles
                ON users_roles.role_id = roles.id
                WHERE username = ?1 AND name = ?2
            ").unwrap().exists(params![username, "admin"]);



            match result {
                Ok(x) => {
                    if x {



                        return ("HTTP/1.1 200 OK".to_string(), fs::read_to_string("www/admin.html").unwrap_or("admin".to_string()));
                    } else {
                        return ("HTTP/1.1 400 BAD REQUEST".to_string(), "No authentication".to_string());
                    }
                },
                Err(e) => {
                    return ("HTTP/1.1 400 BAD REQUEST".to_string(), e.to_string());
                }
            }

        }

    } else {
        return ("HTTP/1.1 400 BAD REQUEST".to_string(), "No authentication".to_string());
    }
}