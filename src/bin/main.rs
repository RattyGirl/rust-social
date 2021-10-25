use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rust_social::ThreadPool;
use rusqlite::{Connection};

mod router;
mod user;
mod httprequest;

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
    conn.execute("drop table users", []);
    conn.execute("create table users (
                id integer primary key,
                username text not null unique,
                hashedpw text not null
         )", [],
    );
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_obj = httprequest::create_request(buffer);

    let (status_line, body) =
        match request_obj.uri.as_str() {
            "/login" => user::login(request_obj, buffer),
            "/login" => user::register(request_obj, buffer),
            _ => ("HTTP/1.1 404 NOT FOUND".to_string(), fs::read_to_string("404.html").unwrap_or("404".to_string()))
        };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        body.len(),
        body
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
