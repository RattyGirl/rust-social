#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::fmt;
use std::str::Split;


pub struct Request {
    pub req_type: TYPE,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub parameters: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn create_emp_req() -> Self {
        Self {
            req_type: TYPE::GET,
            uri: String::new(),
            headers: Default::default(),
            cookies: Default::default(),
            parameters: Default::default(),
            body: String::new()
        }
    }

    pub fn new(buffer: [u8; 1024]) -> Option<Self> {
        let mut request = String::new();

        for c in buffer {
            if c == 0 {
                continue;
            }
            request.push(char::from(c));
        }
        if request.contains("HTTP/1.1") {
            let body_splitheader: Vec<&str> = request.split("\r\n\r\n").collect();
            let headers = body_splitheader[0];
            let body = if body_splitheader.len() == 2 { body_splitheader[1] } else { "" };
            let header_lines: Split<&str> = headers.split("\r\n");


            let mut req_obj = Self {
                req_type: TYPE::GET,
                uri: String::new(),
                headers: HashMap::new(),
                cookies: HashMap::new(),
                parameters: HashMap::new(),
                body: String::new(),
            };
            // read headers
            for (counter, line) in header_lines.into_iter().enumerate() {
                if counter == 0 {
                    //     status line
                    let status_line: Vec<&str> = line.split(' ').collect();
                    req_obj.req_type = match status_line[0] {
                        "GET" => TYPE::GET,
                        "POST" => TYPE::POST,
                        _ => TYPE::GET,
                    };
                    // uri with parametesr
                    let uristuff: Vec<&str> = status_line[1].split('?').collect();
                    req_obj.uri = uristuff[0].to_string();
                    if uristuff.len() > 1 {
                        {
                            //     parameters
                            let parameters: Vec<&str> = uristuff[1].split('&').collect();
                            for parameter in parameters {
                                if !parameter.is_empty() {
                                    let vec: Vec<&str> = parameter.split('=').collect();
                                    if vec.len() == 2 {
                                        req_obj.parameters.insert(vec[0].to_string(), vec[1].to_string());
                                    } else {
                                        req_obj.parameters.insert(vec[0].to_string(), String::new());
                                    }
                                }
                            }
                        }
                    }
                } else {
                    let line: Vec<&str> = line.split(": ").collect();
                    if line[0].eq("Cookie") {
                        let cookies: Vec<&str> = line[1].split("; ").collect();
                        for cookie in cookies {
                            let cookie: Vec<&str> = cookie.split('=').collect();
                            req_obj
                                .cookies
                                .insert(cookie[0].to_string(), cookie[1].to_string());
                        }
                    } else {
                        req_obj
                            .headers
                            .insert(line[0].to_string(), line[1].to_string());
                    }
                }
            }

            req_obj.body = body.to_string();


            Some(req_obj)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TYPE {
    GET,
    POST,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Response {
    status_code: Option<i32>,
    body: String,
    headers: HashMap<String, String>,
}

impl Response {

    pub fn with_code(&mut self, code: i32) -> &mut Response {
        self.status_code = Some(code);
        self
    }

    pub fn with_header(&mut self, key: String, value: String) -> &mut Response {
        self.headers.insert(key, value);
        self
    }

    pub fn with_body(&mut self, body: String) -> &mut Response {
        self.body = body;
        self
    }

    pub fn get_full(&self) -> String {
        let mut headers = String::new();
        for header in &self.headers {
            headers.push_str(format!("{}: {}\r\n", header.0, header.1).as_str());
        }
        format!(
            "HTTP/1.1 {}\r\n{}Content-Length: {}\r\n\r\n{}",
            self.status_code.unwrap_or(404),
            headers,
            self.body.len(),
            self.body
        )
    }
}

impl Clone for Response {
    fn clone(&self) -> Self {
        Response {
            status_code: self.status_code,
            body: self.body.clone(),
            headers: self.headers.clone()
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status_code: None,
            body: "".to_string(),
            headers: HashMap::new()
        }
    }
}