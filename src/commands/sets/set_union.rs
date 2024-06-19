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
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match union(store, &self.keys, usize::MAX) {
            SetOp::New(members) => Ok(Response::Set(members)),
            SetOp::ValueRef(value) => Ok(Response::ValueRef(value)),
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
