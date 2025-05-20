use serde_json::{json, Value};

fn check_identity(id: &str) {
    let valid = id == "server" || id == "client" || {
        id.len() == 5 && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) // {client_id}
    };

    if !valid {
        panic!("Invalid identity: '{}'. Must be 'server', 'client', or [a-z0-9]{{5}}", id);
    }
}

fn compress_identity(id: &str) -> &str {
    match id {
        "server" => "s",
        "client" => "c",
        _ => id, // {client_id} will remain same
    }
}

pub fn build_request(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    json!({
        "f": compress_identity(from),
        "t": compress_identity(to),
        "y": "g", // y -> type, g -> request
        "i": id,
        "p": {
            "m": method,
            "p": params
        } // p -> playload
    }).to_string()
}

pub fn build_response(from: &str,to: &str,id: &str,result: &str,code_or_receipt: &str,) -> String {
    check_identity(from);
    check_identity(to);

    // only "success"/"fail" -> "s"/"f"
    let r = match result {
        "success" => "s",
        "fail" => "f",
        _ => panic!("Invalid result: '{}'. Must be 'success' or 'fail'", result),
    };

    json!({
        "f": compress_identity(from),
        "t": compress_identity(to),
        "y": "r", // r -> response
        "i": id,
        "p": {
            "r": r,
            "c": code_or_receipt
        }
    })
    .to_string()
}

pub fn build_event(from: &str,to: &str,id: &str,method: &str,params: Value,) -> String {
    check_identity(from);
    check_identity(to);

    json!({
        "f": compress_identity(from),
        "t": compress_identity(to),
        "y": "e", // e -> event
        "i": id,
        "p": {
            "m": method,
            "p": params
        }
    })
    .to_string()
}
