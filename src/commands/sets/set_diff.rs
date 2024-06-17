use crate::commands::prelude::*;
use crate::storage::{diff, SetOp};

struct SetDiff {
    keys: Vec<String>,
}

impl SetDiff {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for SetDiff {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match diff(store, &self.keys, usize::MAX) {
            SetOp::Set(members) => client.write_set(&members, registry),
            SetOp::SetRef(members) => client.write_set(members, registry),
            SetOp::Empty => client.write_empty_set(registry),
            SetOp::WrongType => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetDiffParser {}

impl SetDiffParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetDiffParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        if input.has_next() {
            Ok(Box::new(SetDiff::new(input.rest()?)))
        } else {
            Err("Missing SDIFF keys".to_string())
        }
    }
}
