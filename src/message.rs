use serde_json::Value;
use crate::{format::dev, format::prod, get_mode, FormatMode};
use crate::format::common;

fn build_request(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    common::check_identity(from);
    common::check_identity(to);
    match get_mode() {
        FormatMode::Dev => dev::build_request(from, to, id, method, params),
        FormatMode::Prod => prod::build_request(from, to, id, method, params),
    }
}

fn build_response(from: &str, to: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    common::check_identity(from);
    common::check_identity(to);
    match get_mode() {
        FormatMode::Dev => dev::build_response(from, to, id, result, code_or_receipt),
        FormatMode::Prod => prod::build_response(from, to, id, result, code_or_receipt),
    }
}

fn build_event(from: &str, to: &str, id: &str, method: &str, params: Value) -> String {
    common::check_identity(from);
    common::check_identity(to);
    match get_mode() {
        FormatMode::Dev => dev::build_event(from, to, id, method, params),
        FormatMode::Prod => prod::build_event(from, to, id, method, params),
    }
}

pub fn build_request_from_server_to_client(client_id: &str, id: &str, method: &str, params: Value) -> String {
    build_request("server", client_id, id, method, params)
}

pub fn build_request_from_client_to_server(client_id: &str, id: &str, method: &str, params: Value) -> String {
    build_request(client_id, "server", id, method, params)
}

pub fn build_request_from_client_to_client(sender_id: &str, target_client_id: &str, id: &str, method: &str, params: Value) -> String {
    build_request(sender_id, target_client_id, id, method, params)
}

pub fn build_response_from_server_to_client(client_id: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    build_response("server", client_id, id, result, code_or_receipt)
}

pub fn build_response_from_client_to_server(client_id: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    build_response(client_id, "server", id, result, code_or_receipt)
}

pub fn build_response_from_client_to_client(sender_id: &str, target_client_id: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    build_response(sender_id, target_client_id, id, result, code_or_receipt)
}

pub fn build_event_from_server_to_client(id: &str, method: &str, params: Value) -> String {
    build_event("server", "client", id, method, params)
}