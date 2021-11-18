mod view;

use core::fmt;
use std::collections::HashMap;
use std::str::Split;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use rusqlite::{Connection, params};

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// HTTP REQUEST

pub struct Request {
    pub req_type: TYPE,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub parameters: HashMap<String, String>,
    pub body: String,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TYPE {
    GET,
    POST,
}

pub fn create_emp_req() -> Request {
    Request {
        req_type: TYPE::GET,
        uri: "".to_string(),
        headers: Default::default(),
        cookies: Default::default(),
        parameters: Default::default(),
        body: "".to_string()
    }
}

pub fn create_request(buffer: [u8; 1024]) -> Option<Request> {
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


        let mut req_obj = Request {
            req_type: TYPE::GET,
            uri: "".to_string(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            parameters: HashMap::new(),
            body: "".to_string(),
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
                                    req_obj.parameters.insert(vec[0].to_string(), "".to_string());
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

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// models

pub struct User {
    pub username: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Option<Self> {

        let salt: String = rand::random::<u128>().to_string();
        let mut hasher = Sha3::sha3_512();
        hasher.input_str(env!("secret"));
        hasher.input_str(username);
        hasher.input_str(password);
        hasher.input_str(salt.as_str());

        let hashedpw = hasher.result_str();

        let conn = Connection::open("rust-social.db").unwrap();
        match conn.execute(
            "INSERT INTO users (username, hashedpw, salt) VALUES (?1, ?2, ?3)",
            rusqlite::params![username, hashedpw, salt],
        ) {
            Ok(x) => {
                if x == 0 {
                    None
                } else {
                //     user created in db
                    Some(User {
                        username: username.to_string()
                    })
                }
            }
            Err(e) => {
                println!("Error adding user {} to the database\nError: {}", username, e);
                None
            }
        }
    }

    pub fn login(username: &str, password: &str) -> Option<Self> {

        struct UserRow {
            username: String,
            hashedpw: String,
            salt: String
        }

        let conn = Connection::open("rust-social.db").unwrap();
        match conn.query_row("SELECT * FROM users WHERE username = ?1",
            params![username],
            |row| Ok(UserRow {
                username: row.get(0)?,
                hashedpw: row.get(1)?,
                salt: row.get(2)?,
            })) {
            Ok(userrow) => {

                let mut hasher = Sha3::sha3_512();
                hasher.input_str(env!("secret"));
                hasher.input_str(username);
                hasher.input_str(password);
                hasher.input_str(userrow.salt.as_str());

                if hasher.result_str().eq(userrow.hashedpw.as_str()) {
                //     password correct
                    Some(User {
                        username: userrow.username
                    })
                } else {
                    None
                }

            }
            Err(_) => {
                None
            }
        }
    }

    pub fn generate_token(&self) -> Option<String> {
        let session_token: String = rand::random::<u128>().to_string();
        let conn = Connection::open("rust-social.db").unwrap();
        match conn.execute(
            "INSERT INTO sessions (username, token) VALUES (?1, ?2)",
            rusqlite::params![self.username, session_token],
        ) {
            Ok(x) => {
                if x == 0 {
                    None
                } else {
                    Some(session_token)
                }
            }
            Err(e) => {
                println!("Error generating session for user {} to the database\nError: {}", self.username, e);
                None
            }
        }
    }

    pub fn get_if_valid(token: &str) -> Option<Self> {
        // TODO check time
        let conn = Connection::open("rust-social.db").unwrap();
        let row: Result<String, rusqlite::Error> = conn.query_row(
            "SELECT * FROM sessions where token = ?1",
            rusqlite::params![token],
            |row| row.get(1)
        );
        match row {
            Ok(username) => {
                Some(User {
                    username
                })
            }
            Err(e) => {
                println!("{}",e);
                None
            }
        }
    }

    pub fn does_user_have_role(&self, role: String) -> bool {
        let conn = Connection::open("rust-social.db").unwrap();

        let x = conn
            .prepare(
                "
            SELECT * FROM users_roles
            JOIN roles
            ON users_roles.role_id = roles.id
            WHERE username = ?1 AND name = ?2
        ",
            )
            .unwrap()
            .exists(rusqlite::params![self.username, role]);

        x.unwrap_or(false)

    }

}

pub struct Post {
    pub id: i64,
    pub author: String,
    pub content: String,
    pub posted_time: String,
}
