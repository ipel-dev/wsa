use serde_json::Value;
use crate::{format::dev, format::prod, get_mode, FormatMode};

pub fn build_request(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    match get_mode() {
        FormatMode::Dev => dev::build_request(from, to, id, method, params),
        FormatMode::Prod => prod::build_request(from, to, id, method, params),
    }
}

pub fn build_response(from: &str, to: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    match get_mode() {
        FormatMode::Dev => dev::build_response(from, to, id, result, code_or_receipt),
        FormatMode::Prod => prod::build_response(from, to, id, result, code_or_receipt),
    }
}

pub fn build_event(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    match get_mode() {
        FormatMode::Dev => dev::build_event(from, to, id, method, params),
        FormatMode::Prod => prod::build_event(from, to, id, method, params),
    }
}
