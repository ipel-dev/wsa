use wsa::{set_mode, FormatMode, build_request};
use serde_json::json;

fn main() {
    set_mode(FormatMode::Prod); // or FormatMode::Prod

    let msg = build_request(
        "client1",
        "server",
        "2025-05-19T12:00:00Z",
        "auth@v1/login",
        json!({ "username": "abc", "password": "123" })
    );

    println!("{}", msg);
}
