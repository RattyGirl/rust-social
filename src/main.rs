use rusqlite::{Connection};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rust_social::{ThreadPool, DB_LOCATION, SERVER_ADDRESS, make_view, User};
use crate::request::{Request, Response, TYPE};

mod request;
mod user;
mod admin;
mod posts;

fn main() {
    let conn = Connection::open(DB_LOCATION).unwrap();
    let table_gen = generate_tables(&conn);
    match table_gen {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error found when generating tables: {}", e);
        }
    }
    conn.close().unwrap();

    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
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
fn generate_tables(conn: &Connection) -> Result<bool, rusqlite::Error> {
    conn.execute("drop table if exists users_roles", [])?;
    conn.execute("drop table if exists roles", [])?;
    conn.execute("drop table if exists posts", [])?;
    conn.execute("drop table if exists roles", [])?;
    conn.execute("drop table if exists sessions", [])?;
    conn.execute("drop table if exists users", [])?;

    conn.execute(
        "create table users (
                username text not null unique,
                hashedpw text not null,
                salt text not null
         )",
        [],
    )?;

    conn.execute(
        "create table roles (
                id integer primary key AUTOINCREMENT,
                name text not null
         )",
        [],
    )?;

    conn.execute(
        "create table posts (
                id integer primary key AUTOINCREMENT,
                author text not null references users(username),
                content text not null,
                posted_time text not null
         )",
        [],
    )?;

    conn.execute(
        "create table users_roles (
        username text not null references users(username),
        role_id int not null references roles(id),
        PRIMARY KEY (username, role_id)
        )",
        [],
    )?;

    conn.execute(
    "create table sessions (
          id integer primary key AUTOINCREMENT,
          username text not null references users(username),
          token text not null
    )", [],)?;

    // data

    conn.execute("insert into roles (name) VALUES ('admin');", [])?;
    conn.execute("insert into roles (name) VALUES ('moderator');", [])?;
    conn.execute("insert into roles (name) VALUES ('user');", [])?;
    conn.execute("insert into roles (name) VALUES ('banned');", [])?;

    let lab_rat = User::new("rat", "rat");
    match lab_rat {
        None => {}
        Some(u) => {
            u.add_role("admin")?;
            u.add_role("banned")?;
        }
    }

    conn.execute(
        "INSERT INTO posts \
        (author, content, posted_time) \
        VALUES ('rat','aaaaa','2021-11-11 13:18:30')", [],
    )?;

    Ok(true)
}

#[allow(clippy::unused_io_amount)]
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];


    stream.read(&mut buffer).unwrap();

    let request_obj = match Request::new(buffer) {
        None => {
            Request::create_emp_req()
        }
        Some(x) => {
            x
        }
    };

    let response = get_response(&request_obj);

    stream.write_all(response.get_full().as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn get_response(request_obj: &Request) -> Response {
    match (request_obj.uri.to_ascii_lowercase().as_str(), request_obj.req_type) {
        ("/", TYPE::GET) => posts::home_get(request_obj),
        ("/login", TYPE::POST) => user::login_post(request_obj),

        ("/register", TYPE::POST) => user::register_post(request_obj),
        ("/register", TYPE::GET) => user::register_get(request_obj),

        ("/post", TYPE::POST) => posts::post_post(request_obj),

        ("/admin", TYPE::GET) => admin::admin_get(request_obj),


        (_,_) => Response::default().with_code(404).with_body(make_view!("404.social").to_string()).clone(),
    }
}
