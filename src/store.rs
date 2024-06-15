use std::collections::HashMap;
use std::ops::Add;
use std::time::Duration;

use chrono::prelude::*;
use priority_queue::PriorityQueue;

pub struct Store {
    values: HashMap<String, String>,
    expirations: PriorityQueue<String, DateTime<Utc>>
}

impl Store {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            expirations: PriorityQueue::new()
        }
    }

    pub fn set(&mut self, key: &String, value: &String) {
        self.values.insert(key.clone(), value.clone());
    }

    pub fn contains_key(&mut self, key: &String) -> bool {
        self.values.contains_key(key)
    }

    pub fn get(&mut self, key: &String) -> Option<&String> {
        self.values.get(key)
    }

    pub fn keep_forever(&mut self, key: &String) {
        self.expirations.remove(key);
    }

    pub fn expire_at(&mut self, key: &String, at: &DateTime<Utc>) {
        self.expirations.push(key.clone(), at.clone());
    }

    pub fn expire_items(&mut self) {
        let now = Utc::now();
        
        while let Some((key, at)) = self.expirations.peek() {
            if now >= *at {
                self.values.remove(key);
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
    pub fn new_at(key: String, at: DateTime<Utc>) -> Self {
        Self { key, at }
    }

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
