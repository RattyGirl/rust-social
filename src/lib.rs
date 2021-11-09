mod view;

use core::fmt;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

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

pub fn create_request(buffer: [u8; 1024]) -> Request {
    let mut request = String::new();

    for c in buffer {
        if c == 0 {
            continue;
        }
        request.push(char::from(c));
    }
    println!("{}",request);
    let body_splitheader: Vec<&str> = request.split("\r\n\r\n").collect();
    let headers = body_splitheader[0];
    let body = if body_splitheader.len() == 2 {body_splitheader[1]} else {""};
    let header_lines: Vec<&str> = headers.split("\r\n").collect();

    let mut req_obj = Request {
        req_type: TYPE::GET,
        uri: "".to_string(),
        headers: HashMap::new(),
        cookies: HashMap::new(),
        parameters: HashMap::new(),
        body: "".to_string(),
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
            // uri with parametesr
            let uristuff: Vec<&str> = line[1].split('?').collect();
            req_obj.uri = uristuff[0].to_string();
            if uristuff.len() > 1 {
                {
                //     parameters
                    let parameters: Vec<&str> = uristuff[1].split('&').collect();
                    for parameter in parameters {
                        if !parameter.is_empty() {
                            let vec: Vec<&str> = parameter.split("=").collect();
                            req_obj.parameters.insert(vec[0].to_string(), vec[1].to_string());
                        }
                    }
                }
            }
        } else {
            let line: Vec<&str> = line.split(": ").collect();
            if line[0].eq("Cookie") {
                let cookies: Vec<&str> = line[1].split("; ").collect();
                for cookie in cookies {
                    let cookie: Vec<&str> = cookie.split("=").collect();
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

// models

pub struct User {
    pub username: String,
}

pub struct Post {
    pub id: i64,
    pub author: String,
    pub content: String,
    pub posted_time: String,
}
