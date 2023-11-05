use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

pub struct Value {
    pub value: String,
    timestamp: u128,
    expiry: Option<u64>,
}

pub enum KvStatus {
    Found(String),
    KeyNotFound,
    KeyExpired,
}

pub struct KvStore {
    map: HashMap<String, Value>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, expiry: Option<u64>) {
        let timestamp = current_time();

        let value = Value {
            value,
            timestamp,
            expiry,
        };
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> KvStatus {
        match self.map.get(&key) {
            Some(value) => {
                let now = current_time();

                match value.expiry {
                    Some(expiry) => {
                        if now - value.timestamp > expiry as u128 {
                            KvStatus::KeyExpired
                        } else {
                            KvStatus::Found(value.value.clone())
                        }
                    }
                    None => KvStatus::Found(value.value.clone()),
                }
            }
            None => KvStatus::KeyNotFound,
        }
    }
}

fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}