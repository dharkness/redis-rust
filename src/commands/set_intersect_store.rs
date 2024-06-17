use crate::storage::{intersect, Intersect};

use super::prelude::*;

struct SetIntersectStore {
    from: Vec<String>,
    to: String,
}

impl SetIntersectStore {
    pub fn new(from: Vec<String>, to: String) -> Self {
        Self { from, to }
    }
}

impl Apply for SetIntersectStore {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        let intersection = match intersect(store, &self.from, usize::MAX) {
            Intersect::Set(members) => members,
            Intersect::SetRef(members) => members.clone(),
            Intersect::Empty => return client.write_empty_set(registry),
            Intersect::WrongType => return client.write_simple_error(WRONG_TYPE, registry),
        };
        let len = intersection.len() as i64;

        store.set(&self.to, Value::new_set(intersection));
        client.write_integer(len, registry)
    }
}

pub struct SetIntersectStoreParser {}

impl SetIntersectStoreParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetIntersectStoreParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let to = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(SetIntersectStore::new(input.rest()?, to)))
        } else {
            Err("Missing SINTERSTORE keys".to_string())
        }
    }
}
