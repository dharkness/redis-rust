use std::collections::HashMap;

pub struct Store {
    values: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.values.get(&key)
    }
}
