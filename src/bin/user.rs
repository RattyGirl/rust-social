pub struct User {
    id: i32,
    username: String,
    hashedpw: String
}

pub fn login(request: super::httprequest::Request, buffer: [u8; 1024]) -> (String, String) {
    ("HTTP/1.1 404 NOT FOUND\r\nSet-Cookie: rat=ratttttt".to_string(), "hey logged in, or are you".to_string())
//     Login time
}
pub fn register(request: super::httprequest::Request, buffer: [u8; 1024]) -> (String, String) {
    ("HTTP/1.1 404 NOT FOUND\r\nSet-Cookie: rat=ratttttt".to_string(), "register time".to_string())
//     Login time
}