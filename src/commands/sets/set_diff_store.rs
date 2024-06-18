use crate::commands::prelude::*;
use crate::storage::{diff, SetOp};

struct SetDiffStore {
    from: Vec<String>,
    to: String,
}

impl SetDiffStore {
    pub fn new(from: Vec<String>, to: String) -> Self {
        Self { from, to }
    }
}

impl Apply for SetDiffStore {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        let diff = match diff(store, &self.from, usize::MAX) {
            SetOp::Set(members) => members,
            SetOp::SetRef(members) => members.clone(),
            SetOp::Empty => return Ok(Response::EmptySet),
            SetOp::WrongType => return Err(Error::WrongType),
        };
        let len = diff.len();

        store.set(&self.to, Value::from(diff));
        Ok(Response::Usize(len))
    }
}

pub struct SetDiffStoreParser {}

impl SetDiffStoreParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetDiffStoreParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let to = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(SetDiffStore::new(input.rest()?, to)))
        } else {
            Err(Error::MissingArgument(
                "SDIFFSTORE".to_string(),
                "source".to_string(),
            ))
        }
    }
}
