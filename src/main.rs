use serde_json::json;
use wsa::{set_mode, FormatMode, build_request};

fn main() {
    set_mode(FormatMode::Prod); // or FormatMode::Prod

    let msg = build_request(
        "server",
        "client",
        "",
        "auth@v1/login",
        json!(["abc", "123456"])
    );

    println!("{}", msg);
}
