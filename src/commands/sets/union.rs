use crate::commands::prelude::*;
use crate::storage::{SetOp, union};

struct Union {
    keys: Vec<String>,
}

impl Union {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for Union {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match union(store, &self.keys, usize::MAX) {
            SetOp::New(members) => Ok(Response::Set(members)),
            SetOp::ValueRef(value) => Ok(Response::ValueRef(value)),
            SetOp::Empty => Ok(Response::EmptySet),
            SetOp::WrongType => Err(Error::WrongType),
        }
    }
}

pub struct UnionParser {}

impl UnionParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for UnionParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        if input.has_next() {
            Ok(Box::new(Union::new(input.rest()?)))
        } else {
            Err(Error::MissingArgument(
                "SUNION".to_string(),
                "keys".to_string(),
            ))
        }
    }
}
