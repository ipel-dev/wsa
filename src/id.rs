use rand::{thread_rng, Rng};
use std::collections::{HashSet, HashMap};
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::{Value, Map};

struct Pool {
    ids: HashSet<String>,
    storage: HashMap<String, Value>,
}

lazy_static! {
    pub static ref POOLS: Mutex<HashMap<String, Pool>> = Mutex::new(HashMap::new());
}

const ID_LEN: usize = 5;
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

pub fn pool_exists(name: &str) -> bool {
    let pools = POOLS.lock().unwrap();
    pools.contains_key(name)
}

pub fn create_pool(name: &str) -> bool {
    let mut pools = POOLS.lock().unwrap();
    if pools.contains_key(name) {
        panic!("Pool '{}' already exists", name);
    }
    pools.insert(name.to_string(), Pool {
        ids: HashSet::new(),
        storage: HashMap::new(),
    });
    true
}

pub fn delete_pool(name: &str) -> bool {
    let mut pools = POOLS.lock().unwrap();
    if !pools.contains_key(name) {
        panic!("Pool '{}' does not exist", name);
    }
    pools.remove(name);
    true
}

pub fn generate_msg_id(pool_name: &str) -> String {
    let mut pools = POOLS.lock().unwrap();
    let pool = pools.get_mut(pool_name).unwrap_or_else(|| panic!("Pool '{}' does not exist", pool_name));
    let mut rng = thread_rng();
    loop {
        let id: String = (0..ID_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        if !pool.ids.contains(&id) {
            pool.ids.insert(id.clone());
            pool.storage.insert(id.clone(), Value::Object(Map::new()));
            return id;
        }
    }
}

pub fn set_msg_data(pool_name: &str, msg_id: &str, data: Value) -> bool {
    let mut pools = POOLS.lock().unwrap();
    let pool = pools.get_mut(pool_name).unwrap_or_else(|| panic!("Pool '{}' does not exist", pool_name));
    if !pool.storage.contains_key(msg_id) {
        panic!("msg_id '{}' does not exist in pool '{}'", msg_id, pool_name);
    }
    pool.storage.insert(msg_id.to_string(), data);
    true
}

pub fn get_msg_data(pool_name: &str, msg_id: &str) -> Value {
    let pools = POOLS.lock().unwrap();
    let pool = pools.get(pool_name).unwrap_or_else(|| panic!("Pool '{}' does not exist", pool_name));
    pool.storage.get(msg_id)
        .cloned()
        .unwrap_or_else(|| panic!("msg_id '{}' does not exist in pool '{}'", msg_id, pool_name))
}

pub fn clear_msg_id(pool_name: &str, msg_id: &str) -> bool {
    let mut pools = POOLS.lock().unwrap();
    let pool = pools.get_mut(pool_name).unwrap_or_else(|| panic!("Pool '{}' does not exist", pool_name));
    if !pool.ids.contains(msg_id) || !pool.storage.contains_key(msg_id) {
        panic!("msg_id '{}' does not exist in pool '{}'", msg_id, pool_name);
    }
    pool.ids.remove(msg_id);
    pool.storage.remove(msg_id);
    true
}