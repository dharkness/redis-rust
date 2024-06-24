use crate::commands::prelude::*;
use crate::storage::{Random, random_members};

struct RandomMembers {
    key: String,
    count: usize,
    dupes: bool,
}

impl RandomMembers {
    pub fn new(key: String, count: usize, dupes: bool) -> Self {
        Self { key, count, dupes }
    }
}

impl Apply for RandomMembers {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match random_members(store, &self.key, self.count, self.dupes) {
            Random::Single(member) => Ok(Response::BulkString(member.clone())),
            Random::Elements(members) => Ok(Response::List(members)),
            Random::Empty => Ok(Response::EmptyList),
            Random::NotSet => Ok(Response::Null),
            Random::WrongType => Err(Error::WrongType),
        }
    }
}

pub struct RandomMembersParser {}

impl RandomMembersParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for RandomMembersParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;

        if input.has_next() {
            let count = input.next_i64()?;
            if count < 0 {
                Ok(Box::new(RandomMembers::new(key, -count as usize, true)))
            } else {
                Ok(Box::new(RandomMembers::new(key, count as usize, false)))
            }
        } else {
            Ok(Box::new(RandomMembers::new(key, 1, false)))
        }
    }
}
