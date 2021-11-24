use rust_social::{Request, make_view, User};
use rusqlite::Connection;

pub fn home_get(_request: &Request) -> (String, String) {

    (
        "HTTP/1.1 200 OK".to_string(),
        make_view!("home.html",,
        ("{posts}", get_all_posts().as_str())
        )
    )
}

#[derive(Debug)]
#[allow(dead_code)]
struct Post {
    id: i32,
    author: String,
    content: String,
    time: String
}

fn get_all_posts() -> String {
    let mut out: String = String::new();
    let conn = Connection::open(DB_LOCATION).unwrap();

    let mut stmt = conn.prepare("SELECT id, author, content, posted_time FROM posts ORDER BY id DESC").unwrap();
    let posts_iter = stmt.query_map([], |row| {
        Ok(Post {
            id: row.get(0)?,
            author: row.get(1)?,
            content: row.get(2)?,
            time: row.get(3)?,
        })
    }).unwrap();

    for post in posts_iter {
        match post {
            Ok(p) => {
                out.push_str(make_view!("post.html").replace("{username}", &p.author)
                    .replace("{content}", &p.content)
                    .replace("{time}", &p.time).as_str());
            }
            Err(e) => {
                println!("Error with Post {:?}", e);
            }
        }
    }

    out
}

pub fn post_get(_request: &Request) -> (String, String) {
    (
        "HTTP/1.1 200 OK".to_string(),
        make_view!("composepost.html").to_string()
    )
}

pub fn post_post(request: &Request) -> (String, String) {
    let parsed = json::parse(request.body.as_str());
    match parsed {
        Ok(v) => {
            if !v["text"].is_null() {
                // TODO XSS
                match User::get_if_valid(request.cookies.get("token").unwrap_or(&String::new())) {
                    Some(user) => {
                        let conn = Connection::open(DB_LOCATION).unwrap();
                        match conn.execute(
                            "INSERT INTO posts (author, content, posted_time) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
                            rusqlite::params![user.username, v["text"].as_str().unwrap()],
                        ) {
                            Ok(_) => {
                                (
                                    "HTTP/1.1 200 OK".to_string(),
                                    make_view!("homeredirect.html").to_string(),
                                )
                            },
                            Err(e) => {
                                println!("An error occurred making a post: {}", e);
                                (
                                    "HTTP/1.1 200 OK".to_string(),
                                    make_view!("postalert.html",,("{role}", "danger"),
                                    ("{innertext}", "Invalid message")),
                                )
                            }
                        }
                    },
                    None => {
                        (
                            "HTTP/1.1 200 OK".to_string(),
                            make_view!("postalert.html",,("{role}", "danger"),
                            ("{innertext}", "Please login")),
                        )
                    }
                }
            } else {
                (
                    "HTTP/1.1 200 OK".to_string(),
                    make_view!("postalert.html",,("{role}", "danger"),
                    ("{innertext}", "Your message is empty")),
                )
            }
        }
        Err(_) => (
            "HTTP/1.1 200 OK".to_string(),
            make_view!("postalert.html",,("{role}", "danger"),
            ("{innertext}", "Invalid message")),
        ),
    }
}
