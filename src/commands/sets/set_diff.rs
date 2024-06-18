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
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        match diff(store, &self.keys, usize::MAX) {
            SetOp::Set(members) => Ok(Response::Set(members)),
            SetOp::SetRef(members) => Ok(Response::Set(members.clone())),
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
