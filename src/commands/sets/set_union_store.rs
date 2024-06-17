use crate::commands::prelude::*;
use crate::storage::{SetOp, union};

struct SetUnionStore {
    from: Vec<String>,
    to: String,
}

impl SetUnionStore {
    pub fn new(from: Vec<String>, to: String) -> Self {
        Self { from, to }
    }
}

impl Apply for SetUnionStore {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        let union = match union(store, &self.from, usize::MAX) {
            SetOp::Set(members) => members,
            SetOp::SetRef(members) => members.clone(),
            SetOp::Empty => return client.write_empty_set(registry),
            SetOp::WrongType => return client.write_simple_error(WRONG_TYPE, registry),
        };
        let len = union.len() as i64;

        store.set(&self.to, Value::new_set(union));
        client.write_integer(len, registry)
    }
}

pub struct SetUnionStoreParser {}

impl SetUnionStoreParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetUnionStoreParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let to = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(SetUnionStore::new(input.rest()?, to)))
        } else {
            Err("Missing SUNIONSTORE keys".to_string())
        }
    }
}
