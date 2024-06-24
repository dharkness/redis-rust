use crate::commands::prelude::*;
use crate::storage::{diff, SetOp};

struct Diff {
    keys: Vec<String>,
}

impl Diff {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for Diff {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match diff(store, &self.keys, usize::MAX) {
            SetOp::New(members) => Ok(Response::Set(members)),
            SetOp::ValueRef(value) => Ok(Response::ValueRef(value)),
            SetOp::Empty => Ok(Response::EmptySet),
            SetOp::WrongType => Err(Error::WrongType),
        }
    }
}

pub struct DiffParser {}

impl DiffParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for DiffParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        if input.has_next() {
            Ok(Box::new(Diff::new(input.rest()?)))
        } else {
            Err(Error::MissingArgument(
                "SDIFF".to_string(),
                "keys".to_string(),
            ))
        }
    }
}
