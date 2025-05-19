use rand::{thread_rng, Rng};
use std::collections::{HashSet, HashMap};
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::{Value, Map};

lazy_static! {
    pub static ref MSG_POOL: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    pub static ref MSG_STORAGE: Mutex<HashMap<String, Value>> = Mutex::new(HashMap::new());
}

const ID_LEN: usize = 5;
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate_msg_id() -> String {
    let mut pool = MSG_POOL.lock().unwrap();
    let mut storage = MSG_STORAGE.lock().unwrap();
    let mut rng = thread_rng();
    loop {
        let id: String = (0..ID_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        if !pool.contains(&id) {
            pool.insert(id.clone());
            storage.insert(id.clone(), Value::Object(Map::new()));
            return id;
        }
    }
}

pub fn set_msg_data(msg_id: &str, data: Value) -> bool {
    let mut storage = MSG_STORAGE.lock().unwrap();
    if !storage.contains_key(msg_id) {
        panic!("msg_id '{}' does not exist", msg_id);
    }
    storage.insert(msg_id.to_string(), data);
    true
}

pub fn get_msg_data(msg_id: &str) -> Value {
    let storage = MSG_STORAGE.lock().unwrap();
    storage.get(msg_id)
        .cloned()
        .unwrap_or_else(|| panic!("msg_id '{}' does not exist", msg_id))
}

pub fn clear_msg_id(msg_id: &str) -> bool {
    let mut pool = MSG_POOL.lock().unwrap();
    let mut storage = MSG_STORAGE.lock().unwrap();
    if !pool.contains(msg_id) || !storage.contains_key(msg_id) {
        panic!("msg_id '{}' does not exist", msg_id);
    }
    pool.remove(msg_id);
    storage.remove(msg_id);
    true
}
