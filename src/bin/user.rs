use super::httprequest;
use rusqlite::{Connection, params};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn login(request: httprequest::Request, buffer: [u8; 1024]) -> (String, String) {
    ("HTTP/1.1 404 NOT FOUND\r\nSet-Cookie: rat=ratttttt".to_string(), "hey logged in, or are you".to_string())
//     Login time
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
                    match conn.execute(
                        "INSERT INTO users (username, hashedpw) VALUES (?1, ?2)",
                        params![v["username"].as_str(), v["password"].as_str()]
                    ) {
                        Ok(_) => {
                            let mut hasher = Sha3::sha3_512();
                            hasher.input_str(env!("secret"));
                            hasher.input_str(v["username"].as_str().unwrap());
                            hasher.input_str(v["password"].as_str().unwrap());

                            return (
                                "HTTP/1.1 200 OK\r\nSet-Cookie: token=".to_string() + hasher.result_str().as_str(),
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
//     Login time
}