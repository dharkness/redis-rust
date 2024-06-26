use crate::commands::prelude::*;
use crate::storage::{diff, SetOp};

struct DiffStore {
    from: Vec<String>,
    to: String,
}

impl DiffStore {
    pub fn new(from: Vec<String>, to: String) -> Self {
        Self { from, to }
    }
}

impl Apply for DiffStore {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        let diff = match diff(store, &self.from, usize::MAX) {
            SetOp::New(members) => members,
            SetOp::ValueRef(value) => value.expect_set().clone(),
            SetOp::Empty => return Ok(Response::EmptySet),
            SetOp::WrongType => return Err(Error::WrongType),
        };
        let len = diff.len();

        store.set(&self.to, Value::from(diff));
        Ok(Response::Usize(len))
    }
}

pub struct DiffStoreParser {}

impl DiffStoreParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for DiffStoreParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let to = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(DiffStore::new(input.rest()?, to)))
        } else {
            Err(Error::MissingArgument(
                "SDIFFSTORE".to_string(),
                "source".to_string(),
            ))
        }
    }
}
