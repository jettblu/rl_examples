use std::collections::HashMap;

pub trait Store {
    fn generate_id(&self, state: usize, action: usize) -> String {
        format!("{}-{}", state, action)
    }
    fn store_float(&mut self, key: String, value: f64);
    fn get_float(&self, key: &String) -> f64;
    fn new() -> Self;
}

pub struct MemoryStore {
    store: HashMap<String, f64>,
}

impl Store for MemoryStore {
    fn store_float(&mut self, key: String, value: f64) {
        self.store.insert(key, value);
    }
    fn get_float(&self, key: &String) -> f64 {
        match self.store.get(key) {
            Some(value) => *value,
            None => 0.0,
        }
    }
    fn new() -> MemoryStore {
        MemoryStore {
            store: HashMap::new(),
        }
    }
}
