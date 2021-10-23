use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use rust_social::ThreadPool;
use rusqlite::{Connection};

mod user;

fn main() {
    let conn = Connection::open("rust-social.db").unwrap_or(Connection::open_in_memory().unwrap());

    generate_tables(&conn);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn generate_tables(conn: &Connection) {
    conn.execute("create table users (
                id integer primary key,
                username text not null unique,
                hashedpw text not null
         )", [],
    );
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let mut request = String::new();

    for c in buffer {
        if c.eq(&b'\r') {
            break
        }
        request.push(char::from(c));
    }
    let request: Vec<&str> = request.split(' ').collect();
    if request.len() == 3 {
        let request = (request[0], request[1], request[2]);
    //     valid request, now to send to place
        let (status_line, body) =
            if(request.1.eq("/login")) {
                user::login(request.0, buffer)
            } else {
                ("HTTP/1.1 404 NOT FOUND".to_string(), fs::read_to_string("404.html").unwrap())
            };

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            body.len(),
            body
        );

        stream.write(response.as_bytes()).unwrap();
    } else {
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            "HTTP/1.1 404 NOT FOUND",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
    }

    // let (status_line, filename) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else if buffer.starts_with(sleep) {
    //     thread::sleep(Duration::from_secs(5));
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else if(buffer.starts_with(b"GET /login HTTP/1.1\r\n")) {
    //     user::login()
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };


    stream.flush().unwrap();
}
