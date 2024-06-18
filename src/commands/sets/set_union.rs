use crate::commands::prelude::*;
use crate::storage::{SetOp, union};

struct SetUnion {
    keys: Vec<String>,
}

impl SetUnion {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for SetUnion {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        match union(store, &self.keys, usize::MAX) {
            SetOp::Set(members) => Ok(Response::Set(members)),
            SetOp::SetRef(members) => Ok(Response::Set(members.clone())),
            SetOp::Empty => Ok(Response::EmptySet),
            SetOp::WrongType => Err(Error::WrongType),
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        if input.has_next() {
            Ok(Box::new(SetUnion::new(input.rest()?)))
        } else {
            Err(Error::MissingArgument(
                "SUNION".to_string(),
                "keys".to_string(),
            ))
        }
    }
}
