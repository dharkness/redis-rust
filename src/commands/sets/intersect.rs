use crate::commands::prelude::*;
use crate::storage::{intersect, SetOp};

struct Intersect {
    keys: Vec<String>,
}

impl Intersect {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for Intersect {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match intersect(store, &self.keys, usize::MAX) {
            SetOp::New(members) => Ok(Response::Set(members)),
            SetOp::ValueRef(value) => Ok(Response::ValueRef(value)),
            SetOp::Empty => Ok(Response::EmptySet),
            SetOp::WrongType => Err(Error::WrongType),
        }
    }
}

pub struct IntersectParser {}

impl IntersectParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for IntersectParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        if input.has_next() {
            Ok(Box::new(Intersect::new(input.rest()?)))
        } else {
            Err(Error::MissingArgument(
                "SINTER".to_string(),
                "keys".to_string(),
            ))
        }
    }
}
