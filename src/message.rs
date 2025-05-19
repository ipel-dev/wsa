use serde_json::Value;

use crate::id;
use crate::format;

pub fn build_request_from_server_to_client(client_id: &str /*这里使用下面的 id::generate_msg_id 生成 msg_id: &str*/, method: &str, version: /*num here */ params: Value) -> String {
    id::generate_msg_id
    format::build_request()
}


/* 
pub fn build_request_from_client_to_server(client_id: &str, id: &str, method: &str, params: Value) -> String {
    format::build_request(client_id, "server", id, method, params)
}

pub fn build_request_from_client_to_client(sender_id: &str, target_client_id: &str, id: &str, method: &str, params: Value) -> String {
    format::build_request(sender_id, target_client_id, id, method, params)
}







pub fn build_response_from_server_to_client(client_id: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    format::build_response("server", client_id, id, result, code_or_receipt)
}

pub fn build_response_from_client_to_server(client_id: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    format::build_response(client_id, "server", id, result, code_or_receipt)
}

pub fn build_response_from_client_to_client(sender_id: &str, target_client_id: &str, id: &str, result: &str, code_or_receipt: &str) -> String {
    format::build_response(sender_id, target_client_id, id, result, code_or_receipt)
}

pub fn build_event_from_server_to_client(id: &str, method: &str, params: Value) -> String {
    format::build_event("server", "client", id, method, params)
}
*/