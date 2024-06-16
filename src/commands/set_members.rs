use super::prelude::*;

struct SetMembers {
    key: String,
}

impl SetMembers {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for SetMembers {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => client.write_set(members, registry),
            IfKindResult::NotSet => client.write_null(registry),
            _ => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetMembersParser {}

impl SetMembersParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetMembersParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(SetMembers::new(input.next_string()?)))
    }
}
