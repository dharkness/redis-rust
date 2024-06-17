use crate::commands::prelude::*;
use crate::storage::{diff, SetOp};

struct SetDiffStore {
    from: Vec<String>,
    to: String,
}

impl SetDiffStore {
    pub fn new(from: Vec<String>, to: String) -> Self {
        Self { from, to }
    }
}

impl Apply for SetDiffStore {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        let diff = match diff(store, &self.from, usize::MAX) {
            SetOp::Set(members) => members,
            SetOp::SetRef(members) => members.clone(),
            SetOp::Empty => return client.write_empty_set(registry),
            SetOp::WrongType => return client.write_simple_error(WRONG_TYPE, registry),
        };
        let len = diff.len() as i64;

        store.set(&self.to, Value::new_set(diff));
        client.write_integer(len, registry)
    }
}

pub struct SetDiffStoreParser {}

impl SetDiffStoreParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetDiffStoreParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let to = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(SetDiffStore::new(input.rest()?, to)))
        } else {
            Err("Missing SDIFFSTORE keys".to_string())
        }
    }
}
