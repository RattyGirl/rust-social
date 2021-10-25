use std::fmt;
use std::collections::HashMap;

pub struct Request {
    pub req_type: TYPE,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: String
}

pub enum TYPE {
    GET,
    POST
}

pub fn create_request(buffer: [u8; 1024]) -> Request {

    let mut request = String::new();

    for c in buffer {
        if c == 0 {
            continue
        }
        request.push(char::from(c));
    }
    let body_splitheader: Vec<&str> = request.split("\r\n\r\n").collect();
    let headers = body_splitheader[0];
    let body = body_splitheader[1];
    let header_lines: Vec<&str> = headers.split("\r\n").collect();

    let mut req_obj = Request {
        req_type: TYPE::GET,
        uri: "".to_string(),
        headers: HashMap::new(),
        cookies: HashMap::new(),
        body: "".to_string()
    };
    // read headers
    let mut counter = 0;
    for line in header_lines {
        if counter == 0 {
        //     status line
            let line: Vec<&str> = line.split(' ').collect();
            req_obj.req_type = match line[0] {
                "GET" => TYPE::GET,
                "POST" => TYPE::POST,
                _ => TYPE::GET,
            };
            req_obj.uri = line[1].to_string();
        } else {
            let line: Vec<&str> = line.split(": ").collect();
            if line[0].eq("Cookie") {
                let cookies: Vec<&str> = line[1].split("; ").collect();
                for cookie in cookies {
                    let cookie: Vec<&str> = cookie.split("=").collect();
                    req_obj.cookies.insert(cookie[0].to_string(), cookie[1].to_string());
                }
            } else {
                req_obj.headers.insert(line[0].to_string(), line[1].to_string());
            }
        }
        counter = counter + 1;
    }

    req_obj.body = body.to_string();

    return req_obj;
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}