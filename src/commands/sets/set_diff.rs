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
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match diff(store, &self.keys, usize::MAX) {
            SetOp::New(members) => Ok(Response::Set(members)),
            SetOp::ValueRef(value) => Ok(Response::ValueRef(value)),
            SetOp::Empty => Ok(Response::EmptySet),
            SetOp::WrongType => Err(Error::WrongType),
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        if input.has_next() {
            Ok(Box::new(SetDiff::new(input.rest()?)))
        } else {
            Err(Error::MissingArgument(
                "SDIFF".to_string(),
                "keys".to_string(),
            ))
        }
    }
}
