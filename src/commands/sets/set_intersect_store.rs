use crate::commands::prelude::*;
use crate::storage::{intersect, SetOp};

struct SetIntersectStore {
    from: Vec<String>,
    to: String,
}

impl SetIntersectStore {
    pub fn new(from: Vec<String>, to: String) -> Self {
        Self { from, to }
    }
}

impl Apply for SetIntersectStore {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        let intersection = match intersect(store, &self.from, usize::MAX) {
            SetOp::Set(members) => members,
            SetOp::SetRef(members) => members.clone(),
            SetOp::Empty => return Ok(Response::EmptySet),
            SetOp::WrongType => return Err(Error::WrongType),
        };
        let len = intersection.len();

        store.set(&self.to, Value::from(intersection));
        Ok(Response::Usize(len))
    }
}

pub struct SetIntersectStoreParser {}

impl SetIntersectStoreParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetIntersectStoreParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let to = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(SetIntersectStore::new(input.rest()?, to)))
        } else {
            Err(Error::MissingArgument(
                "SINTERSTORE".to_string(),
                "source".to_string(),
            ))
        }
    }
}
