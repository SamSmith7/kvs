use std::collections::HashMap;

/// A simple key-value store structure that is built on top of a HashMap
#[derive(Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// Create a new KvStore
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// Retreive and item from the store by it's key
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// Set an item in the store, passing in it's key and value
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Remove a key from the store and delete the associated data
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
