use crate::storage::{pop_random_members, Random};

use super::prelude::*;

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
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match pop_random_members(store, &self.key, self.count) {
            Random::Single(member) => client.write_bulk_string(&member, registry),
            Random::Elements(members) => client.write_array(&members, registry),
            Random::Empty => client.write_empty_array(registry),
            Random::NotSet => client.write_null(registry),
            Random::WrongType => client.write_simple_error(WRONG_TYPE, registry),
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;

        if input.has_next() {
            Ok(Box::new(SetPop::new(key, input.next_u64()? as usize)))
        } else {
            Ok(Box::new(SetPop::new(key, 1)))
        }
    }
}
