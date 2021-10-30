use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rust_social::ThreadPool;
use rusqlite::{Connection};

pub mod router;
pub mod user;
pub mod httprequest;
pub mod admin;

fn main() {
    let conn = Connection::open("rust-social.db").unwrap();

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

#[allow(unused_must_use)]
fn generate_tables(conn: &Connection) {

    conn.execute("drop table users_roles", []);
    conn.execute("drop table roles", []);
    conn.execute("drop table users", []);


    conn.execute("create table users (
                username text not null unique,
                hashedpw text not null,
                salt text
         )", [],
    );

    conn.execute("drop table roles", []);
    conn.execute("create table roles (
                id integer primary key AUTOINCREMENT,
                name text not null
         )", [],
    );

    conn.execute("create table users_roles (
        username text not null references users(username),
        role_id int not null references roles(id),
        PRIMARY KEY (username, role_id)
    )", []);

    // data

    conn.execute("insert into roles (name) VALUES ('admin');", []);
    conn.execute("insert into roles (name) VALUES ('moderator');", []);
    conn.execute("insert into roles (name) VALUES ('user');", []);
    conn.execute("insert into roles (name) VALUES ('banned');", []);

    conn.execute("insert into users (username, hashedpw) VALUES ('rat', 'd98d547fbcb9546985057ad654ef1ea81ae1a950c2adbcbd4a9216e13300080e40e2d316eb80d97446be5f20f01f9bc7f214d8e6797a21ebba950295b34cb5d5');", []);

    conn.execute("insert into users_roles (username, role_id) VALUES ('rat',1)", []);
    conn.execute("insert into users_roles (username, role_id) VALUES ('rat',4)", []);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_obj = httprequest::create_request(buffer);

    let (status_line, body) =
        match request_obj.uri.as_str() {
            "/login" => user::login(request_obj, buffer),
            "/register" => user::register(request_obj, buffer),
            "/admin" => admin::admin(request_obj, buffer),
            _ => ("HTTP/1.1 404 NOT FOUND".to_string(), fs::read_to_string("www/404.html").unwrap_or("404".to_string()))
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
