use super::httprequest;

pub fn admin(request: httprequest::Request, _buffer: [u8; 1024]) -> (String, String) {
    // SELECT username, name FROM users_roles
    // JOIN roles
    // ON users_roles.role_id = roles.id
    // WHERE username = 'rat' AND name = 'admin'
    // ;


    ("HTTP/1.1 400 BAD REQUEST".to_string(), "boop".to_string())

}