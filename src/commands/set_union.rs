use crate::storage::{union, Union};

use super::prelude::*;

struct SetUnion {
    keys: Vec<String>,
}

impl SetUnion {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for SetUnion {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match union(store, &self.keys, usize::MAX) {
            Union::Set(members) => client.write_set(&members, registry),
            Union::SetRef(members) => client.write_set(members, registry),
            Union::Empty => client.write_empty_set(registry),
            Union::WrongType => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetUnionParser {}

impl SetUnionParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetUnionParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        if input.has_next() {
            Ok(Box::new(SetUnion::new(input.rest()?)))
        } else {
            Err("Missing SUNION keys".to_string())
        }
    }
}
