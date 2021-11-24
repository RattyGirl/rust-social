mod view;
mod request;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use rusqlite::{Connection, params};

pub const DB_LOCATION: &str = "rust-social.db";
pub const SERVER_ADDRESS: &str = "127.0.0.1:7878";

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

impl User {
    pub fn new(username: &str, password: &str) -> Option<Self> {

        let salt: String = rand::random::<u128>().to_string();
        let mut hasher = Sha3::sha3_512();
        hasher.input_str(env!("secret"));
        hasher.input_str(username);
        hasher.input_str(password);
        hasher.input_str(salt.as_str());

        let hashedpw = hasher.result_str();

        let conn = Connection::open(DB_LOCATION).unwrap();
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

        let conn = Connection::open(DB_LOCATION).unwrap();
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
        let conn = Connection::open(DB_LOCATION).unwrap();
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
        let conn = Connection::open(DB_LOCATION).unwrap();
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
        let conn = Connection::open(DB_LOCATION).unwrap();

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
