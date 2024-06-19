use crate::commands::prelude::*;
use crate::storage::{pop_random_members, Random};

struct SetPop {
    key: String,
    count: usize,
}

impl SetPop {
    pub fn new(key: String, count: usize) -> Self {
        Self { key, count }
    }
}

impl Apply for SetPop {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match pop_random_members(store, &self.key, self.count) {
            Random::Single(member) => Ok(Response::BulkString(member.clone())),
            Random::Elements(members) => Ok(Response::List(members)),
            Random::Empty => Ok(Response::EmptyList),
            Random::NotSet => Ok(Response::Null),
            Random::WrongType => Err(Error::WrongType),
        }
    }
}

pub struct SetPopParser {}

impl SetPopParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetPopParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(SetPop::new(key, input.next_u64()? as usize)))
        } else {
            Ok(Box::new(SetPop::new(key, 1)))
        }
    }
}
