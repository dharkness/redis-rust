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
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        match intersect(store, &self.keys, usize::MAX) {
            SetOp::Set(members) => Ok(Response::Set(members)),
            SetOp::SetRef(members) => Ok(Response::Set(members.clone())),
            SetOp::Empty => Ok(Response::EmptySet),
            SetOp::WrongType => Err(Error::WrongType),
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        if input.has_next() {
            Ok(Box::new(SetIntersect::new(input.rest()?)))
        } else {
            Err(Error::MissingArgument(
                "SINTER".to_string(),
                "keys".to_string(),
            ))
        }
    }
}
