use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use chrono::prelude::*;
use itertools::Itertools;
use priority_queue::PriorityQueue;

use super::{Kind, Pattern, Value};

pub struct Store {
    values: HashMap<String, Value>,
    expirations: PriorityQueue<String, DateTime<Utc>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            expirations: PriorityQueue::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: Value) -> Option<Value> {
        self.values.insert(key.to_string(), value)
    }

    pub fn set_if_kind(&mut self, kind: Kind, key: &str, value: Value) -> IfKindResult<Value> {
        match self.values.entry(key.to_string()) {
            Occupied(mut entry) => {
                if entry.get().kind() == kind {
                    IfKindResult::Matched(entry.insert(value))
                } else {
                    IfKindResult::NotMatched
                }
            }
            Vacant(entry) => {
                entry.insert(value);
                IfKindResult::NotSet
            }
        }
    }

    pub fn rename(&mut self, key: &str, new_key: &str) -> bool {
        if let Some(value) = self.values.remove(key) {
            self.values.insert(new_key.to_string(), value);
            self.expirations.remove(key);
            self.expirations.remove(new_key);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.expirations.remove(key);
        self.values.remove(key).is_some()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn keys(&self, pattern: &Pattern) -> Vec<&str> {
        self.values
            .keys()
            .filter(|key| pattern.matches(key))
            .map(|key| key.as_str())
            .collect_vec()
    }

    pub fn kind(&self, key: &str) -> Option<Kind> {
        self.values.get(key).map(|value| value.kind())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn get_if_kind(&self, kind: Kind, key: &str) -> IfKindResult<&Value> {
        if let Some(value) = self.values.get(key) {
            if value.kind() == kind {
                IfKindResult::Matched(value)
            } else {
                IfKindResult::NotMatched
            }
        } else {
            IfKindResult::NotSet
        }
    }

    pub fn get_and_remove(&mut self, key: &str) -> Option<Value> {
        self.expirations.remove(key);
        self.values.remove(key)
    }

    pub fn get_and_remove_if_kind(&mut self, kind: Kind, key: &str) -> IfKindResult<Value> {
        match self.values.entry(key.to_string()) {
            Occupied(entry) => {
                if entry.get().kind() == kind {
                    IfKindResult::Matched(entry.remove())
                } else {
                    IfKindResult::NotMatched
                }
            }
            Vacant(_) => IfKindResult::NotSet,
        }
    }

    pub fn copy(&mut self, source: &str, destination: &str) -> bool {
        if let Some(value) = self.get(source) {
            self.values.insert(destination.to_string(), value.clone());
            true
        } else {
            false
        }
    }

    pub fn is_volatile(&self, key: &str) -> bool {
        self.expirations.get(key).is_some()
    }

    pub fn expires(&self, key: &str) -> Option<&DateTime<Utc>> {
        self.expirations.get_priority(key)
    }

    pub fn persist(&mut self, key: &str) -> bool {
        self.expirations.remove(key).is_some()
    }

    pub fn expire_at(&mut self, key: &str, at: &DateTime<Utc>) {
        self.expirations.push(key.to_string(), *at);
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

pub enum IfKindResult<T> {
    NotSet,
    NotMatched,
    Matched(T),
}
