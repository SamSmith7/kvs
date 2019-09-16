use bincode;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::BufWriter;
use std::fs::{File, OpenOptions};
use std::path::Path;
use crate::{KvError, Result};


#[derive(Serialize, Deserialize, Debug)]
enum Action {
    Get,
    Remove,
    Set
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    action: Action,
    key: String,
    value: String,
}

/// A simple key-value store structure that is built on top of a HashMap
// #[derive(Default)]
pub struct KvStore {
    handle: File,
}

impl KvStore {
    /// Create a new KvStore
    // pub fn new() -> KvStore {
    //     KvStore {
    //         data: HashMap::new(),
    //     }
    // }

    /// Open an on disc file and use it to boostrap the store
    pub fn open(path: &Path) -> Result<KvStore> {

        // let handle = File::open(path)?;
        let handle = OpenOptions::new().append(true).read(true).open(path)?;
        let store = KvStore {
            handle
        };

        Ok(store)
    }

    /// Retreive and item from the store by it's key
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(None)
    }

    /// Set an item in the store, passing in it's key and value
    pub fn set(&self, key: String, value: String) -> Result<()> {
        let command = Command {
            action: Action::Set,
            key: key,
            value: value
        };
        let encoded: Vec<u8> = bincode::serialize(&command)?;
        self.handle.write_all(encoded)?;

        Ok(())
    }

    /// Remove a key from the store and delete the associated data
    pub fn remove(&mut self, key: String) -> Result<()> {
        // self.data.remove(&key);
        Ok(())
    }
}
