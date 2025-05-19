use serde_json::{json, Value};
use super::common::{compress_identity};

pub fn build_request(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    json!({
        "f": compress_identity(from),
        "t": compress_identity(to),
        "y": "g",
        "i": id,
        "P": {
            "p": {
                "m": method,
                "p": params
            }
        }
    }).to_string()
}

pub fn build_response(from: &str, to: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    let payload = match result {
        "success" => json!({ "r": "s", "c": code_or_receipt }),
        "fail" => json!({ "r": "f", "c": code_or_receipt }),
        _ => json!({}),
    };
    json!({
        "f": compress_identity(from),
        "t": compress_identity(to),
        "y": "r",
        "i": id,
        "P": {
            "p": payload
        }
    }).to_string()
}

pub fn build_event(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    json!({
        "f": compress_identity(from),
        "t": compress_identity(to),
        "y": "e",
        "i": id,
        "P": {
            "p": {
                "m": method,
                "p": params
            }
        }
    }).to_string()
}
