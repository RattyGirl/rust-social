use rust_social::{Request, make_view, User};
use rusqlite::Connection;

pub fn home_get(request: &Request) -> (String, String) {
    (
        "HTTP/1.1 200 OK".to_string(),
        make_view!("home.html",,
        ("{posts}", get_all_posts().as_str())
        ).to_string()
    )
}

#[derive(Debug)]
struct Post {
    id: i32,
    author: String,
    content: String,
    time: String
}

fn get_all_posts() -> String {
    let mut out: String = String::new();
    let conn = Connection::open("rust-social.db").unwrap();

    let mut stmt = conn.prepare("SELECT id, author, content, posted_time FROM posts").unwrap();
    let posts_iter = stmt.query_map([], |row| {
        Ok(Post {
            id: row.get(0)?,
            author: row.get(1)?,
            content: row.get(2)?,
            time: row.get(3)?,
        })
    }).unwrap();

    for post in posts_iter {
        if post.is_ok() {
            let post = post.unwrap();
            out.push_str(make_view!("post.html").replace("{username}", &post.author)
                .replace("{content}", &post.content)
                .replace("{time}", &post.time).as_str());
        }
    }

    return out;
}

pub fn post_get(request: &Request) -> (String, String) {
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
                match User::get_if_valid(request.cookies.get("token").unwrap_or(&"".to_string())) {
                    Some(user) => {
                        let conn = Connection::open("rust-social.db").unwrap();
                        match conn.execute(
                            "INSERT INTO posts (author, content, posted_time) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
                            rusqlite::params![user.username, v["text"].as_str().unwrap()],
                        ) {
                            Ok(_) => {
                                (
                                    "HTTP/1.1 200 OK".to_string(),
                                    make_view!("postalert.html",,("{role}", "success"),
                                    ("{innertext}", v["text"].as_str().unwrap())).to_string(),
                                )
                            },
                            Err(e) => {
                                println!("An error occurred making a post: {}", e);
                                (
                                    "HTTP/1.1 200 OK".to_string(),
                                    make_view!("postalert.html",,("{role}", "danger"),
                                    ("{innertext}", "Invalid message")).to_string(),
                                )
                            }
                        }
                    },
                    None => {
                        (
                            "HTTP/1.1 200 OK".to_string(),
                            make_view!("postalert.html",,("{role}", "danger"),
                            ("{innertext}", "Please login")).to_string(),
                        )
                    }
                }
            } else {
                (
                    "HTTP/1.1 200 OK".to_string(),
                    make_view!("postalert.html",,("{role}", "danger"),
                    ("{innertext}", "Your message is empty")).to_string(),
                )
            }
        }
        Err(_) => (
            "HTTP/1.1 200 OK".to_string(),
            make_view!("postalert.html",,("{role}", "danger"),
            ("{innertext}", "Invalid message")).to_string(),
        ),
    }
}
