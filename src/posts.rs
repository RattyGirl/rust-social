use rust_social::{Request, make_view};

pub fn home_get(request: &Request) -> (String, String) {
    (
        "HTTP/1.1 200 OK".to_string(),
        make_view!("home.html",,
        ("{postcontent}",make_view!("composepost.html")),
        ("{posts}", make_view!("post.html"))
        ).to_string()
    )
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
                (
                    "HTTP/1.1 200 OK".to_string(),
                    make_view!("postalert.html",,("{role}", "success"),
                    ("{innertext}", v["text"].as_str().unwrap())).to_string(),
                )
            } else {
                (
                    "HTTP/1.1 200 OK".to_string(),
                    make_view!("postalert.html",,("{role}", "danger"),
                    ("{innertext}", "Failed to parse JSON")).to_string(),
                )
            }
        }
        Err(_) => (
            "HTTP/1.1 200 OK".to_string(),
            make_view!("postalert.html",,("{role}", "danger"),
            ("{innertext}", "Failed to parse JSON")).to_string(),
        ),
    }
}
