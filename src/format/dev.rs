use serde_json::{json, Value};

pub fn build_request(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    json!({
        "from": from,
        "to": to,
        "type": "request",
        "id": id,
        "payload": {
            "method": method,
            "params": params
        }
    }).to_string()
}

pub fn build_response(from: &str, to: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    let payload = match result {
        "success" => json!({ "result": "success", "receipt": code_or_receipt }),
        "fail" => json!({ "result": "fail", "code": code_or_receipt }),
        _ => json!({}),
    };
    json!({
        "from": from,
        "to": to,
        "type": "response",
        "id": id,
        "payload": payload
    }).to_string()
}

pub fn build_event(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    json!({
        "from": from,
        "to": to,
        "type": "event",
        "id": id,
        "payload": {
            "method": method,
            "params": params
        }
    }).to_string()
}
