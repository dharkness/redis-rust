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
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match intersect(store, &self.keys, usize::MAX) {
            SetOp::New(members) => Ok(Response::Set(members)),
            SetOp::ValueRef(value) => Ok(Response::ValueRef(value)),
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
