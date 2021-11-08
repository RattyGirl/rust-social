use rust_social::Request;

pub fn post_post(request: &Request) -> (String, String) {
    (
        "HTTP/1.1 404 NOT FOUND".to_string(),
        format!("{:?}", request.parameters),
    )
}
