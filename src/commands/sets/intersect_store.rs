use crate::commands::prelude::*;
use crate::storage::{intersect, SetOp};

struct IntersectStore {
    from: Vec<String>,
    to: String,
}

impl IntersectStore {
    pub fn new(from: Vec<String>, to: String) -> Self {
        Self { from, to }
    }
}

impl Apply for IntersectStore {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        let intersection = match intersect(store, &self.from, usize::MAX) {
            SetOp::New(members) => members,
            SetOp::ValueRef(value) => value.expect_set().clone(),
            SetOp::Empty => return Ok(Response::EmptySet),
            SetOp::WrongType => return Err(Error::WrongType),
        };
        let len = intersection.len();

        store.set(&self.to, Value::from(intersection));
        Ok(Response::Usize(len))
    }
}

pub struct IntersectStoreParser {}

impl IntersectStoreParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for IntersectStoreParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let to = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(IntersectStore::new(input.rest()?, to)))
        } else {
            Err(Error::MissingArgument(
                "SINTERSTORE".to_string(),
                "source".to_string(),
            ))
        }
    }
}
