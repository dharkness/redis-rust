use crate::commands::prelude::*;
use crate::storage::{Random, random_members};

struct SetRandomMembers {
    key: String,
    count: usize,
    dupes: bool,
}

impl SetRandomMembers {
    pub fn new(key: String, count: usize, dupes: bool) -> Self {
        Self { key, count, dupes }
    }
}

impl Apply for SetRandomMembers {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match random_members(store, &self.key, self.count, self.dupes) {
            Random::Single(member) => client.write_bulk_string(&member, registry),
            Random::Elements(members) => client.write_array(&members, registry),
            Random::Empty => client.write_empty_array(registry),
            Random::NotSet => client.write_null(registry),
            Random::WrongType => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetRandomMembersParser {}

impl SetRandomMembersParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetRandomMembersParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;

        if input.has_next() {
            let count = input.next_i64()?;
            if count < 0 {
                Ok(Box::new(SetRandomMembers::new(key, -count as usize, true)))
            } else {
                Ok(Box::new(SetRandomMembers::new(key, count as usize, false)))
            }
        } else {
            Ok(Box::new(SetRandomMembers::new(key, 1, false)))
        }
    }
}
