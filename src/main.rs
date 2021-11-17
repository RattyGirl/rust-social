use rusqlite::Connection;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rust_social::{ThreadPool, create_request, TYPE};
use rust_social::make_view;

mod user;
mod admin;
mod posts;

fn main() {
    let conn = Connection::open("rust-social.db").unwrap();
    generate_tables(&conn);
    conn.close().unwrap();

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

#[allow(unused_must_use)]
fn generate_tables(conn: &Connection) {
    conn.execute("drop table users_roles", []);
    conn.execute("drop table roles", []);
    conn.execute("drop table posts", []);
    conn.execute("drop table roles", []);
    conn.execute("drop table sessions", []);
    conn.execute("drop table users", []);

    conn.execute(
        "create table users (
                username text not null unique,
                hashedpw text not null,
                salt text not null
         )",
        [],
    );

    conn.execute(
        "create table roles (
                id integer primary key AUTOINCREMENT,
                name text not null
         )",
        [],
    );

    conn.execute(
        "create table posts (
                id integer primary key AUTOINCREMENT,
                author text not null references users(username),
                content text not null,
                posted_time text not null
         )",
        [],
    );

    conn.execute(
        "create table users_roles (
        username text not null references users(username),
        role_id int not null references roles(id),
        PRIMARY KEY (username, role_id)
        )",
        [],
    );

    conn.execute(
    "create table sessions (
          id integer primary key AUTOINCREMENT,
          username text not null references users(username),
          token text not null
    )", [],);

    // data

    conn.execute("insert into roles (name) VALUES ('admin');", []);
    conn.execute("insert into roles (name) VALUES ('moderator');", []);
    conn.execute("insert into roles (name) VALUES ('user');", []);
    conn.execute("insert into roles (name) VALUES ('banned');", []);

    conn.execute("insert into users (username, hashedpw, salt) VALUES ('rat','c7c21c131c46c3e2725bb745de6768baf41ae0366110fc3645bd5bcc50145a3bea3f52323888fac36dbde6e964b1d0678c13116e1193d465bdcaa189d119cc9a', '319152662097124763509187784674982699619');", []);

    conn.execute(
        "insert into users_roles (username, role_id) VALUES ('rat',1)",[],
    );
    conn.execute(
        "insert into users_roles (username, role_id) VALUES ('rat',4)",[],
    );
    conn.execute(
        "INSERT INTO posts (author, content, posted_time) VALUES ('a','rat','2021-11-11 13:18:30')", [],
    );
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_obj = match create_request(buffer) {
        None => {
            rust_social::create_emp_req()
        }
        Some(x) => {
            x
        }
    };

    let (status_line, body) = match (request_obj.uri.to_ascii_lowercase().as_str(), request_obj.req_type) {
        ("/", TYPE::GET) => posts::home_get(&request_obj),
        ("/login", TYPE::POST) => user::login_post(&request_obj),
        ("/register", TYPE::POST) => user::register_post(&request_obj),
        ("/admin", TYPE::GET) => admin::admin_get(&request_obj),
        ("/post", TYPE::GET) => posts::post_get(&request_obj),
        ("/post", TYPE::POST) => posts::post_post(&request_obj),
        (_,_) => ("HTTP/1.1 404 NOT FOUND".to_string(), make_view!("404.html").to_string()),
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
