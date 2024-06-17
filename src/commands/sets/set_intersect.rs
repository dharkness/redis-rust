use crate::commands::prelude::*;
use crate::storage::{intersect, SetOp};

struct SetIntersect {
    keys: Vec<String>,
}

impl SetIntersect {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for SetIntersect {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match intersect(store, &self.keys, usize::MAX) {
            SetOp::Set(members) => client.write_set(&members, registry),
            SetOp::SetRef(members) => client.write_set(members, registry),
            SetOp::Empty => client.write_empty_set(registry),
            SetOp::WrongType => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetIntersectParser {}

impl SetIntersectParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetIntersectParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        if input.has_next() {
            Ok(Box::new(SetIntersect::new(input.rest()?)))
        } else {
            Err("Missing SINTER keys".to_string())
        }
    }
}
