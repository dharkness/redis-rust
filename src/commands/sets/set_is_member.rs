use crate::commands::prelude::*;

struct SetIsMember {
    key: String,
    value: String,
}

impl SetIsMember {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl Apply for SetIsMember {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => {
                client.write_integer(if members.contains(&self.value) { 1 } else { 0 }, registry)
            }
            IfKindResult::NotSet => client.write_zero(registry),
            _ => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetIsMemberParser {}

impl SetIsMemberParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetIsMemberParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(SetIsMember::new(
            input.next_string()?,
            input.next_string()?,
        )))
    }
}

enum When {
    Always,
    Exists,
    NotExists,
}
