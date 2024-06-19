use crate::commands::prelude::*;
use crate::storage::{SetOp, union};

struct SetUnionStore {
    from: Vec<String>,
    to: String,
}

impl SetUnionStore {
    pub fn new(from: Vec<String>, to: String) -> Self {
        Self { from, to }
    }
}

impl Apply for SetUnionStore {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        let union = match union(store, &self.from, usize::MAX) {
            SetOp::New(members) => members,
            SetOp::ValueRef(value) => value.expect_set().clone(),
            SetOp::Empty => return Ok(Response::EmptySet),
            SetOp::WrongType => return Err(Error::WrongType),
        };
        let len = union.len();

        store.set(&self.to, Value::from(union));
        Ok(Response::Usize(len))
    }
}

pub struct SetUnionStoreParser {}

impl SetUnionStoreParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetUnionStoreParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let to = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(SetUnionStore::new(input.rest()?, to)))
        } else {
            Err(Error::MissingArgument(
                "SUNIONSTORE".to_string(),
                "source".to_string(),
            ))
        }
    }
}
