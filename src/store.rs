use std::collections::{BinaryHeap, HashMap};
use std::ops::Add;
use std::time::{Duration};

use chrono::prelude::*;

pub struct Store {
    values: HashMap<String, String>,
    expirations: BinaryHeap<Expiration>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            expirations: BinaryHeap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.expire_items();
        self.values.insert(key, value);
    }

    pub fn set_with_expiration(&mut self, key: String, value: String, ms: usize) {
        self.values.insert(key.clone(), value);
        self.expirations.push(Expiration::new(key, ms));
    }

    pub fn get(&mut self, key: String) -> Option<&String> {
        self.expire_items();
        self.values.get(&key)
    }

    fn expire_items(&mut self) {
        while let Some(expiration) = self.expirations.peek() {
            if expiration.expired() {
                self.values.remove(expiration.key());
                self.expirations.pop();
            } else {
                break;
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Expiration {
    key: String,
    at: DateTime<Utc>,
}

impl Expiration {
    fn new(key: String, ms: usize) -> Self {
        let duration = if ms >= 1000 {
            let secs = ms / 1000;
            Duration::new(secs as u64, (ms - secs * 1000) as u32 * 1000000)
        } else {
            Duration::new(0, ms as u32 * 1000000)
        };
        let at = Utc::now().add(duration);
        println!("will expire {} at {}", key, at.format("%Y-%m-%d %H:%M:%S").to_string());
        Self { key, at }
    }

    fn expired(&self) -> bool {
        let now = Utc::now();
        if now >= self.at {
            println!("expired key {} at {}", self.key, now.format("%Y-%m-%d %H:%M:%S").to_string());
            true
        } else {
            false
        }
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for Expiration {
    /// Expirations are ordered by their expiry time, with the earliest expiry time first.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.at.cmp(&self.at)
    }
}

impl PartialOrd for Expiration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
